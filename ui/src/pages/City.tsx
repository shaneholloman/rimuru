import { useRef, useEffect, useState, useCallback } from "react";
import { useQuery } from "../hooks/useQuery";
import { useStream } from "../hooks/useStream";
import type { Agent } from "../api/types";
import { CityEngine } from "../city/engine";

export default function City() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const engineRef = useRef<CityEngine | null>(null);
  const dragRef = useRef({ dragging: false, lastX: 0, lastY: 0 });

  const { events: streamEvents, connected: wsConnected } = useStream("agents");
  const { data: agents } = useQuery<Agent[]>("/agents", 10000);

  const [agentCount, setAgentCount] = useState(0);
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;

    const rect = container.getBoundingClientRect();
    canvas.width = Math.round(rect.width * window.devicePixelRatio);
    canvas.height = Math.round(rect.height * window.devicePixelRatio);

    const engine = new CityEngine(canvas);
    engineRef.current = engine;
    engine.start();

    const ro = new ResizeObserver(() => {
      const r = container.getBoundingClientRect();
      canvas.width = Math.round(r.width * window.devicePixelRatio);
      canvas.height = Math.round(r.height * window.devicePixelRatio);
    });
    ro.observe(container);

    const onWheel = (e: WheelEvent) => {
      e.preventDefault();
      const cr = canvas.getBoundingClientRect();
      const px = (e.clientX - cr.left) * window.devicePixelRatio;
      const py = (e.clientY - cr.top) * window.devicePixelRatio;
      const step = 0.25 * (window.devicePixelRatio || 1);
      engine.zoomBy(e.deltaY > 0 ? -step : step, px, py);
    };
    canvas.addEventListener("wheel", onWheel, { passive: false });

    return () => {
      engine.stop();
      ro.disconnect();
      canvas.removeEventListener("wheel", onWheel);
    };
  }, []);

  useEffect(() => {
    if (!engineRef.current || streamEvents.length === 0) return;
    for (const evt of streamEvents) {
      engineRef.current.handleStreamEvent(evt);
    }
    setAgentCount(engineRef.current.characters.size);
  }, [streamEvents]);

  useEffect(() => {
    if (agents && engineRef.current) {
      engineRef.current.syncAgents(agents);
      setAgentCount(engineRef.current.characters.size);
    }
  }, [agents]);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    dragRef.current = { dragging: true, lastX: e.clientX, lastY: e.clientY };
  }, []);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!dragRef.current.dragging || !engineRef.current) return;
    const dx = dragRef.current.lastX - e.clientX;
    const dy = dragRef.current.lastY - e.clientY;
    engineRef.current.panBy(dx, dy);
    dragRef.current.lastX = e.clientX;
    dragRef.current.lastY = e.clientY;
  }, []);

  const handleMouseUp = useCallback(() => {
    dragRef.current.dragging = false;
  }, []);

  const handleDoubleClick = useCallback((e: React.MouseEvent) => {
    if (!engineRef.current || !canvasRef.current) return;
    const rect = canvasRef.current.getBoundingClientRect();
    const cx = (e.clientX - rect.left) * window.devicePixelRatio;
    const cy = (e.clientY - rect.top) * window.devicePixelRatio;
    const char = engineRef.current.findCharacterAt(cx, cy);
    if (char) {
      engineRef.current.centerOnCharacter(char.agentId);
      setSelectedAgent(char.agentId);
    } else {
      setSelectedAgent(null);
    }
  }, []);

  const zoomIn = useCallback(() => {
    if (engineRef.current)
      engineRef.current.zoomTo(engineRef.current.camera.zoom + 0.5);
  }, []);

  const zoomOut = useCallback(() => {
    if (engineRef.current)
      engineRef.current.zoomTo(engineRef.current.camera.zoom - 0.5);
  }, []);

  const resetCamera = useCallback(() => {
    if (engineRef.current) engineRef.current.resetCamera();
  }, []);

  const selectedChar = selectedAgent
    ? engineRef.current?.characters.get(selectedAgent)
    : null;

  return (
    <div
      className="relative w-full"
      style={{ height: "calc(100vh - 5rem)" }}
      ref={containerRef}
    >
      <canvas
        ref={canvasRef}
        className="w-full h-full rounded-xl border border-[var(--border)] cursor-grab active:cursor-grabbing"
        style={{ imageRendering: "pixelated" }}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
        onDoubleClick={handleDoubleClick}
      />

      <div className="absolute top-4 right-4 flex flex-col gap-1.5">
        {[
          { label: "+", action: zoomIn },
          { label: "\u2013", action: zoomOut },
        ].map((b) => (
          <button
            key={b.label}
            onClick={b.action}
            className="w-9 h-9 rounded-lg bg-[var(--bg-secondary)]/90 backdrop-blur border border-[var(--border)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] flex items-center justify-center text-lg font-bold transition-colors"
          >
            {b.label}
          </button>
        ))}
        <button
          onClick={resetCamera}
          className="w-9 h-9 rounded-lg bg-[var(--bg-secondary)]/90 backdrop-blur border border-[var(--border)] text-[var(--text-primary)] hover:bg-[var(--bg-tertiary)] flex items-center justify-center transition-colors"
          title="Reset view"
        >
          <svg
            className="w-4 h-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
            />
          </svg>
        </button>
      </div>

      <div className="absolute bottom-4 left-4 flex items-center gap-2">
        <div className="px-3 py-1.5 rounded-lg bg-[var(--bg-secondary)]/90 backdrop-blur border border-[var(--border)] text-sm">
          <span className="font-semibold text-[var(--accent)]">
            {agentCount}
          </span>
          <span className="text-[var(--text-secondary)] ml-1">
            agent{agentCount !== 1 ? "s" : ""} in Tempest
          </span>
        </div>
        {wsConnected && (
          <div className="px-2 py-1 rounded-md bg-green-500/10 border border-green-500/30 text-xs text-green-400 flex items-center gap-1.5">
            <span className="w-1.5 h-1.5 rounded-full bg-green-400 animate-pulse" />
            Live
          </div>
        )}
      </div>

      {selectedChar && (
        <div className="absolute bottom-4 right-4 px-4 py-3 rounded-xl bg-[var(--bg-secondary)]/95 backdrop-blur border border-[var(--border)] text-sm min-w-[180px]">
          <div className="font-semibold text-[var(--text-primary)]">
            {selectedChar.name}
          </div>
          <div className="flex items-center gap-2 mt-1">
            <span
              className={`w-2 h-2 rounded-full ${
                selectedChar.status === "connected" ||
                selectedChar.status === "active"
                  ? "bg-green-400"
                  : selectedChar.status === "idle"
                    ? "bg-yellow-400"
                    : selectedChar.status === "busy"
                      ? "bg-blue-400"
                      : selectedChar.status === "error"
                        ? "bg-red-400"
                        : "bg-gray-400"
              }`}
            />
            <span className="text-xs text-[var(--text-secondary)] capitalize">
              {selectedChar.status}
            </span>
          </div>
          <div className="text-xs text-[var(--text-tertiary)] mt-1 capitalize">
            {selectedChar.characterType} \u2022{" "}
            {selectedChar.state.toLowerCase()}
          </div>
        </div>
      )}

      <div className="absolute top-4 left-4 px-3 py-1.5 rounded-lg bg-[var(--bg-secondary)]/90 backdrop-blur border border-[var(--border)]">
        <span className="text-sm font-bold text-[var(--accent)]">
          Tempest City
        </span>
      </div>
    </div>
  );
}
