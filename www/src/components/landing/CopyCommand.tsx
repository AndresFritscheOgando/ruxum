import { useState } from "react";

const COMMAND = "npx create-ruxum-app@latest";

export default function CopyCommand() {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(COMMAND);
      setCopied(true);
      setTimeout(() => setCopied(false), 2200);
    } catch {
      // Clipboard API unavailable — silent fail
    }
  };

  return (
    <button
      onClick={handleCopy}
      aria-label={copied ? "Copied!" : "Copy installation command"}
      className="flex items-center gap-2 px-4 py-3 w-full max-w-[420px]
                 bg-surface border border-border-2 rounded-none cursor-pointer
                 font-mono text-sm text-left
                 hover:border-border-3 hover:bg-surface-2
                 transition-all duration-200"
    >
      <span className="text-green shrink-0">$</span>
      <span className="text-ink-2 flex-1">{COMMAND}</span>
      <span
        className={`shrink-0 px-2 py-0.5 border rounded-none
                    text-[0.6875rem] uppercase tracking-[0.05em] font-mono
                    transition-all duration-200
                    ${copied
                      ? "border-green text-green"
                      : "border-border-2 text-ink-3"
                    }`}
      >
        {copied ? "✓ copied" : "copy"}
      </span>
    </button>
  );
}
