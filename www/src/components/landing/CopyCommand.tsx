import { useState, useCallback, useEffect, useRef } from "react";

const COMMAND = "npx create-ruxum-app@latest";

export default function CopyCommand() {
  const [copied, setCopied] = useState(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Stable reference — won't cause child re-renders if passed as prop
  const handleCopy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(COMMAND);
      setCopied(true);
      // Clear any in-flight reset before scheduling a new one
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
      timeoutRef.current = setTimeout(() => setCopied(false), 2200);
    } catch {
      // Clipboard API unavailable — silent fail
    }
  }, []);

  // Clear pending reset on unmount to prevent setState on unmounted component
  useEffect(() => {
    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, []);

  return (
    <button
      onClick={handleCopy}
      aria-label={copied ? "Copied!" : "Copy installation command"}
      className={`flex items-center gap-2 px-4 py-3 w-full max-w-[420px]
                 bg-surface border border-border-2 rounded-none cursor-pointer
                 font-mono text-sm text-left
                 hover:border-border-3 hover:bg-surface-2
                 hover:[box-shadow:0_0_0_1px_rgba(255,255,255,0.08),inset_0_0_16px_rgba(0,255,65,0.02)]
                 transition-all duration-200
                 ${copied ? "border-l-2 border-l-orange" : ""}`}
    >
      <span
        className={`shrink-0 [text-shadow:0_0_8px_rgba(0,255,65,0.6)]
                    ${copied ? "text-orange" : "text-green"}`}
      >
        $
      </span>
      <span className="text-ink-2 flex-1">{COMMAND}</span>
      <span
        className={`shrink-0 px-2 py-1 border rounded-none
                    text-[0.6875rem] uppercase tracking-[0.05em] font-mono
                    transition-all duration-200
                    ${copied
                      ? "border-green text-green [box-shadow:0_0_8px_rgba(0,255,65,0.2)]"
                      : "border-border-2 text-ink-3"
                    }`}
      >
        {copied ? "✓ copied" : "copy"}
      </span>
    </button>
  );
}
