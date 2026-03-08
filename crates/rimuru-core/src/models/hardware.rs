use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccelBackend {
    Metal,
    Cuda,
    Rocm,
    CpuArm,
    CpuX86,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vram_mb: u64,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu_cores: u32,
    pub cpu_brand: String,
    pub total_ram_mb: u64,
    pub available_ram_mb: u64,
    pub gpu: Option<GpuInfo>,
    pub backend: AccelBackend,
    pub os: String,
    pub arch: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitLevel {
    Perfect,
    Good,
    Marginal,
    TooTight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModelAdvisory {
    pub model_id: String,
    pub model_name: String,
    pub can_run_locally: bool,
    pub fit_level: FitLevel,
    pub best_quantization: Option<String>,
    pub estimated_vram_mb: Option<u64>,
    pub estimated_tok_per_sec: Option<f64>,
    pub local_equivalent: Option<String>,
    pub api_cost_spent: f64,
    pub potential_savings: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModel {
    pub name: String,
    pub provider: String,
    pub params_b: f64,
    pub context_length: u64,
    pub use_case: String,
    pub architecture: String,
    pub capabilities: Vec<String>,
    pub hf_downloads: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub name: String,
    pub provider: String,
    pub params_b: f64,
    pub context_length: u64,
    pub use_case: String,
    pub architecture: String,
    pub capabilities: Vec<String>,
    pub hf_downloads: u64,
    pub fit_level: FitLevel,
    pub can_run: bool,
    pub best_quantization: Option<String>,
    pub estimated_vram_mb: Option<u64>,
    pub estimated_tok_per_sec: Option<f64>,
}

pub struct LocalEquivalent {
    pub api_model_id: &'static str,
    pub local_name: &'static str,
    pub params_b: f64,
}

pub fn local_equivalents() -> Vec<LocalEquivalent> {
    vec![
        LocalEquivalent {
            api_model_id: "claude-opus-4-6",
            local_name: "Qwen3-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "claude-sonnet-4-6",
            local_name: "Qwen3-14B",
            params_b: 14.0,
        },
        LocalEquivalent {
            api_model_id: "claude-haiku-3-5",
            local_name: "Qwen3-4B",
            params_b: 4.0,
        },
        LocalEquivalent {
            api_model_id: "gpt-4o",
            local_name: "Qwen3-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "gpt-4o-mini",
            local_name: "Qwen3-8B",
            params_b: 8.0,
        },
        LocalEquivalent {
            api_model_id: "o3",
            local_name: "QwQ-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "o3-mini",
            local_name: "Phi-4-reasoning",
            params_b: 14.0,
        },
        LocalEquivalent {
            api_model_id: "gemini-2.5-pro",
            local_name: "Qwen3-235B-A22B",
            params_b: 22.0,
        },
        LocalEquivalent {
            api_model_id: "gemini-2.5-flash",
            local_name: "Qwen3-4B",
            params_b: 4.0,
        },
        LocalEquivalent {
            api_model_id: "deepseek-v3",
            local_name: "DeepSeek-R1-Distill-Qwen-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "deepseek-r1",
            local_name: "DeepSeek-R1-Distill-Qwen-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "kimi-k2.5",
            local_name: "Qwen3-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "glm-5",
            local_name: "Qwen3-32B",
            params_b: 32.0,
        },
        LocalEquivalent {
            api_model_id: "glm-4",
            local_name: "Qwen3-14B",
            params_b: 14.0,
        },
        LocalEquivalent {
            api_model_id: "mistral-large",
            local_name: "Mistral-Small-24B",
            params_b: 24.0,
        },
        LocalEquivalent {
            api_model_id: "codestral",
            local_name: "Devstral-Small-2",
            params_b: 24.0,
        },
        LocalEquivalent {
            api_model_id: "llama-4-maverick",
            local_name: "Llama-4-Maverick-17B-128E",
            params_b: 17.0,
        },
        LocalEquivalent {
            api_model_id: "llama-4-scout",
            local_name: "Llama-4-Scout-17B",
            params_b: 17.0,
        },
    ]
}

pub struct Quantization {
    pub name: &'static str,
    pub bpp: f64,
    pub speed_mult: f64,
}

pub fn quantizations() -> Vec<Quantization> {
    vec![
        Quantization {
            name: "Q8_0",
            bpp: 8.0,
            speed_mult: 0.95,
        },
        Quantization {
            name: "Q6_K",
            bpp: 6.57,
            speed_mult: 1.0,
        },
        Quantization {
            name: "Q4_K_M",
            bpp: 4.83,
            speed_mult: 1.1,
        },
        Quantization {
            name: "Q3_K_S",
            bpp: 3.44,
            speed_mult: 1.15,
        },
        Quantization {
            name: "Q2_K",
            bpp: 2.63,
            speed_mult: 1.2,
        },
    ]
}

pub fn estimate_vram_mb(params_b: f64, bpp: f64) -> u64 {
    ((params_b * bpp / 8.0) * 1.1 * 1024.0) as u64
}

pub fn estimate_tok_per_sec(params_b: f64, backend: AccelBackend, speed_mult: f64) -> f64 {
    let base = match backend {
        AccelBackend::Cuda => 220.0,
        AccelBackend::Metal => 160.0,
        AccelBackend::Rocm => 180.0,
        AccelBackend::CpuArm => 90.0,
        AccelBackend::CpuX86 => 70.0,
    };
    (base / params_b) * speed_mult
}

pub fn assess_fit(
    hw: &HardwareInfo,
    params_b: f64,
) -> (FitLevel, Option<String>, Option<u64>, Option<f64>) {
    let available_vram = hw.gpu.as_ref().map(|g| g.vram_mb).unwrap_or(0);
    let available_ram = hw.available_ram_mb;

    for q in quantizations() {
        let required = estimate_vram_mb(params_b, q.bpp);

        if available_vram > 0 && available_vram >= (required as f64 * 1.3) as u64 {
            let tok_s = estimate_tok_per_sec(params_b, hw.backend, q.speed_mult);
            return (
                FitLevel::Perfect,
                Some(q.name.into()),
                Some(required),
                Some(tok_s),
            );
        }

        if available_vram > 0 && available_vram >= required {
            let tok_s = estimate_tok_per_sec(params_b, hw.backend, q.speed_mult);
            return (
                FitLevel::Good,
                Some(q.name.into()),
                Some(required),
                Some(tok_s),
            );
        }
    }

    let smallest_q = quantizations().into_iter().last().unwrap();
    let smallest_req = estimate_vram_mb(params_b, smallest_q.bpp);

    if available_ram >= smallest_req {
        let cpu_backend = if hw.arch.contains("arm") || hw.arch.contains("aarch64") {
            AccelBackend::CpuArm
        } else {
            AccelBackend::CpuX86
        };
        let tok_s = estimate_tok_per_sec(params_b, cpu_backend, smallest_q.speed_mult);
        return (
            FitLevel::Marginal,
            Some(smallest_q.name.into()),
            Some(smallest_req),
            Some(tok_s),
        );
    }

    (FitLevel::TooTight, None, None, None)
}
