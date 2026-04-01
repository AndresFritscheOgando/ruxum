import { useState, useCallback, useEffect, useRef } from "react";

const COMMAND = "npx create-ruxum-app@latest";

export default function CopyCommand() {
  const [copied, setCopied] = useState(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const handleCopy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(COMMAND);
      setCopied(true);
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
      timeoutRef.current = setTimeout(() => setCopied(false), 2200);
    } catch {
      // Clipboard API unavailable — silent fail
    }
  }, []);

  useEffect(() => {
    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, []);

  return (
    <button
      onClick={handleCopy}
      aria-label={copied ? "Copied!" : "Copy install command"}
      className={`${copied ? "live-wire glow-pulse" : ""}`}
      style={{
        display: "flex",
        alignItems: "center",
        gap: "0.75rem",
        padding: "0.75rem 1rem",
        width: "100%",
        maxWidth: "400px",
        background: "var(--color-surface)",
        border: `1px solid ${copied ? "var(--color-orange)" : "var(--color-border-2)"}`,
        borderRadius: "8px",
        cursor: "pointer",
        fontFamily: "var(--font-mono)",
        fontSize: "0.875rem",
        textAlign: "left",
        transition: "border-color 0.15s ease, background 0.15s ease, box-shadow 0.15s ease",
        boxShadow: copied
          ? "0 0 20px -5px var(--color-orange-molten)"
          : "none",
        position: "relative",
      }}
      onMouseEnter={e => {
        if (!copied) {
          (e.currentTarget as HTMLButtonElement).style.borderColor = "var(--color-border-3)";
          (e.currentTarget as HTMLButtonElement).style.background = "var(--color-surface-2)";
        }
      }}
      onMouseLeave={e => {
        if (!copied) {
          (e.currentTarget as HTMLButtonElement).style.borderColor = "var(--color-border-2)";
          (e.currentTarget as HTMLButtonElement).style.background = "var(--color-surface)";
        }
      }}
    >
      <span style={{ color: "var(--color-ink-4)", flexShrink: 0, userSelect: "none" }}>$</span>
      <span style={{ color: "var(--color-ink-2)", flex: 1 }}>{COMMAND}</span>
      <span
        style={{
          flexShrink: 0,
          padding: "0.2rem 0.5rem",
          border: `1px solid ${copied ? "var(--color-orange)" : "var(--color-border-2)"}`,
          borderRadius: "4px",
          fontSize: "0.625rem",
          textTransform: "uppercase",
          letterSpacing: "0.06em",
          color: copied ? "var(--color-orange-hi)" : "var(--color-ink-3)",
          transition: "all 0.15s ease",
        }}
      >
        {copied ? "✓ copiado" : "copiar"}
      </span>
    </button>
  );
}
