import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";

type StepType = "input" | "question" | "choice" | "success" | "meta";

interface Step {
  type: StepType;
  text: string;
  delay: number;
}

const STEPS: Step[] = [
  { type: "input",    text: "npx create-ruxum-app@latest",   delay: 900 },
  { type: "question", text: "◆  What would you like to scaffold?", delay: 600 },
  { type: "choice",   text: "●  Full-stack (Axum + Next.js)", delay: 700 },
  { type: "question", text: "◆  What is your project named?", delay: 500 },
  { type: "input",    text: "my-app",                         delay: 750 },
  { type: "question", text: "◆  Database (Rust)?",            delay: 500 },
  { type: "choice",   text: "●  PostgreSQL — SQLx",           delay: 700 },
  { type: "question", text: "◆  Add JWT authentication?",     delay: 500 },
  { type: "choice",   text: "●  Yes",                         delay: 600 },
  { type: "question", text: "◆  Next.js extras?",             delay: 500 },
  { type: "choice",   text: "●  TypeScript + Tailwind CSS",   delay: 700 },
  { type: "question", text: "◆  Scaffold this project?",      delay: 500 },
  { type: "choice",   text: "●  Yes",                         delay: 600 },
  { type: "success",  text: "✔  Created my-app in 0.9s",      delay: 1100 },
  { type: "meta",     text: "→  cd my-app && cargo run",      delay: 400 },
];

const RESET_DELAY = 4500;

export default function TerminalWizard() {
  const [currentStep, setCurrentStep] = useState(0);
  const [displayedSteps, setDisplayedSteps] = useState<Step[]>([]);

  useEffect(() => {
    if (currentStep < STEPS.length) {
      const timer = setTimeout(() => {
        setDisplayedSteps((prev) => [...prev, STEPS[currentStep]]);
        setCurrentStep((prev) => prev + 1);
      }, STEPS[currentStep].delay);
      return () => clearTimeout(timer);
    } else {
      const reset = setTimeout(() => {
        setDisplayedSteps([]);
        setCurrentStep(0);
      }, RESET_DELAY);
      return () => clearTimeout(reset);
    }
  }, [currentStep]);

  return (
    <div className="relative overflow-hidden bg-[#0d0d0d] border border-border-2 font-mono text-[0.8125rem]">
      {/* Window chrome */}
      <div className="flex items-center gap-1.5 px-5 py-3.5 border-b border-border">
        <span className="w-2 h-2 rounded-full bg-[#ff5f56]" />
        <span className="w-2 h-2 rounded-full bg-[#ffbd2e]" />
        <span className="w-2 h-2 rounded-full bg-[#27c93f]" />
        <span className="ml-3 text-[0.6875rem] text-white/25 tracking-[0.04em]">
          create-ruxum-app
        </span>
      </div>

      {/* Steps */}
      <div className="px-5 py-5 min-h-[260px]">
        <AnimatePresence initial={false}>
          {displayedSteps.map((step, idx) => (
            <motion.div
              key={idx}
              initial={{ opacity: 0, y: 4 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.18, ease: "easeOut" }}
              className="mb-2 leading-[1.5]"
            >
              {step.type === "input" && (
                <div className="flex gap-2">
                  <span className="text-green shrink-0">$</span>
                  <span className="text-ink font-semibold">{step.text}</span>
                </div>
              )}
              {step.type === "question" && (
                <div className="text-ink-2">{step.text}</div>
              )}
              {step.type === "choice" && (
                <div className="text-orange pl-5">{step.text}</div>
              )}
              {step.type === "success" && (
                <div className="text-green font-semibold mt-3">{step.text}</div>
              )}
              {step.type === "meta" && (
                <div className="text-ink-3 pl-5">{step.text}</div>
              )}
            </motion.div>
          ))}
        </AnimatePresence>

        {currentStep < STEPS.length && (
          <motion.span
            animate={{ opacity: [0, 1, 0] }}
            transition={{ repeat: Infinity, duration: 0.85 }}
            className="inline-block w-2 h-4 bg-ink align-middle ml-0.5"
          />
        )}
      </div>

      {/* CRT vignette */}
      <div
        aria-hidden="true"
        className="crt-vignette absolute inset-0 pointer-events-none z-10"
      />
    </div>
  );
}
