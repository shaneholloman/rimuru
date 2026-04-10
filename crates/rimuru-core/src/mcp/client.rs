use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, oneshot};
use tracing::{debug, info, warn};

use super::types::*;
use crate::error::RimuruError;

type Result<T> = std::result::Result<T, RimuruError>;

pub struct McpClient {
    stdin: Arc<Mutex<tokio::process::ChildStdin>>,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<JsonRpcResponse>>>>,
    next_id: AtomicU64,
    server_info: Option<McpInitializeResult>,
    _child: Child,
}

impl McpClient {
    pub async fn connect(config: &ProxyServerConfig) -> Result<Self> {
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        for (k, v) in &config.env {
            cmd.env(k, v);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| RimuruError::Bridge(format!("Failed to spawn MCP server: {}", e)))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| RimuruError::Bridge("No stdin".to_string()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| RimuruError::Bridge("No stdout".to_string()))?;

        if let Some(stderr) = child.stderr.take() {
            let server_name = config.name.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    debug!("[{}] stderr: {}", server_name, line);
                }
            });
        }

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<JsonRpcResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_clone = pending.clone();

        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut header_buf = String::new();

            loop {
                header_buf.clear();
                let mut content_length: Option<usize> = None;

                loop {
                    let bytes_read = match reader.read_line(&mut header_buf).await {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    if bytes_read == 0 {
                        break;
                    }

                    let line = header_buf.trim_end();
                    if line.is_empty() {
                        break;
                    }

                    if let Some(len_str) = line
                        .strip_prefix("Content-Length:")
                        .or_else(|| line.strip_prefix("content-length:"))
                    {
                        content_length = len_str.trim().parse().ok();
                    }

                    header_buf.clear();
                }

                let body = if let Some(len) = content_length {
                    let mut buf = vec![0u8; len];
                    if reader.read_exact(&mut buf).await.is_err() {
                        break;
                    }
                    String::from_utf8_lossy(&buf).to_string()
                } else {
                    header_buf.trim().to_string()
                };

                if body.is_empty() {
                    continue;
                }

                match serde_json::from_str::<JsonRpcResponse>(&body) {
                    Ok(resp) => {
                        if let Some(id) = resp.id {
                            let mut map = pending_clone.lock().await;
                            if let Some(tx) = map.remove(&id) {
                                let _ = tx.send(resp);
                            }
                        } else {
                            debug!("MCP notification: {}", &body[..body.len().min(200)]);
                        }
                    }
                    Err(_) => {
                        if let Ok(v) = serde_json::from_str::<Value>(&body) {
                            let method = v
                                .get("method")
                                .and_then(|m| m.as_str())
                                .unwrap_or("unknown");
                            if method == "notifications/tools/list_changed" {
                                info!("MCP server signaled tools/list_changed");
                            } else {
                                debug!("MCP notification: {}", method);
                            }
                        }
                    }
                }
            }

            let mut map = pending_clone.lock().await;
            for (id, tx) in map.drain() {
                let _ = tx.send(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: Some(id),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -1,
                        message: "MCP server process exited".to_string(),
                        data: None,
                    }),
                });
            }
        });

        let mut client = Self {
            stdin: Arc::new(Mutex::new(stdin)),
            pending,
            next_id: AtomicU64::new(1),
            server_info: None,
            _child: child,
        };

        client.initialize().await?;
        Ok(client)
    }

    async fn send_notification(&self, method: &str, params: Option<Value>) {
        let body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params.unwrap_or(json!({}))
        });

        let msg = match serde_json::to_string(&body) {
            Ok(m) => m,
            Err(e) => {
                warn!("Failed to serialize notification {}: {}", method, e);
                return;
            }
        };

        let framed = format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg);
        let mut stdin = self.stdin.lock().await;
        if let Err(e) = stdin.write_all(framed.as_bytes()).await {
            warn!("Failed to send notification {}: {}", method, e);
            return;
        }
        let _ = stdin.flush().await;
    }

    async fn send_request(&self, method: &str, params: Option<Value>) -> Result<Value> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        };

        let msg = serde_json::to_string(&request)
            .map_err(|e| RimuruError::Bridge(format!("Serialize error: {}", e)))?;

        let framed = format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg);

        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        let write_result = async {
            let mut stdin = self.stdin.lock().await;
            stdin.write_all(framed.as_bytes()).await?;
            stdin.flush().await
        }
        .await;

        if let Err(e) = write_result {
            self.pending.lock().await.remove(&id);
            return Err(RimuruError::Bridge(format!("Write error: {}", e)));
        }

        let resp = match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
            Ok(Ok(resp)) => resp,
            Ok(Err(_)) => {
                self.pending.lock().await.remove(&id);
                return Err(RimuruError::Bridge("Channel closed".to_string()));
            }
            Err(_) => {
                self.pending.lock().await.remove(&id);
                return Err(RimuruError::Bridge(format!(
                    "Timeout waiting for response to {}",
                    method
                )));
            }
        };

        if let Some(err) = resp.error {
            return Err(RimuruError::Bridge(format!(
                "MCP error {}: {}",
                err.code, err.message
            )));
        }

        Ok(resp.result.unwrap_or(json!(null)))
    }

    async fn initialize(&mut self) -> Result<()> {
        let result = self
            .send_request(
                "initialize",
                Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {
                        "name": "rimuru-mcp-proxy",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })),
            )
            .await?;

        let init_result: McpInitializeResult = serde_json::from_value(result)
            .map_err(|e| RimuruError::Bridge(format!("Invalid initialize response: {}", e)))?;

        self.server_info = Some(init_result);

        self.send_notification("notifications/initialized", Some(json!({})))
            .await;

        Ok(())
    }

    pub async fn tools_list(&self) -> Result<Vec<McpTool>> {
        let result = self.send_request("tools/list", Some(json!({}))).await?;

        let list_result: McpToolsListResult = serde_json::from_value(result)
            .map_err(|e| RimuruError::Bridge(format!("Invalid tools/list response: {}", e)))?;

        Ok(list_result.tools)
    }

    pub async fn tools_call(&self, tool_name: &str, arguments: Value) -> Result<McpToolCallResult> {
        let result = self
            .send_request(
                "tools/call",
                Some(json!({
                    "name": tool_name,
                    "arguments": arguments
                })),
            )
            .await?;

        let call_result: McpToolCallResult = serde_json::from_value(result)
            .map_err(|e| RimuruError::Bridge(format!("Invalid tools/call response: {}", e)))?;

        Ok(call_result)
    }

    pub fn server_info(&self) -> Option<&McpInitializeResult> {
        self.server_info.as_ref()
    }

    pub fn estimate_tokens(value: &Value) -> u64 {
        let s = value.to_string();
        (s.len() as u64) / 4
    }
}
