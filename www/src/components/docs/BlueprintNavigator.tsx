import React, { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";

interface NodeProps {
  id: string;
  label: string;
  x: number;
  y: number;
  icon: string;
  description: string;
  href: string;
  isHovered: boolean;
  onHover: (id: string | null) => void;
}

const Node = ({ id, label, x, y, icon, description, href, isHovered, onHover }: NodeProps) => {
  return (
    <motion.g
      initial={false}
      animate={{ opacity: 1 }}
      onMouseEnter={() => onHover(id)}
      onMouseLeave={() => onHover(null)}
      className="cursor-pointer"
      onClick={() => window.location.href = href}
    >
      {/* Glow Effect */}
      <AnimatePresence>
        {isHovered && (
          <motion.circle
            initial={{ r: 20, opacity: 0 }}
            animate={{ r: 35, opacity: 0.2 }}
            exit={{ r: 20, opacity: 0 }}
            cx={x}
            cy={y}
            fill="var(--color-orange)"
          />
        )}
      </AnimatePresence>

      {/* Main Node */}
      <motion.circle
        animate={{
          r: isHovered ? 28 : 24,
          fill: isHovered ? "var(--color-surface-3)" : "var(--color-surface)",
          stroke: isHovered ? "var(--color-orange)" : "var(--color-border-3)",
        }}
        cx={x}
        cy={y}
        strokeWidth="1.5"
      />

      {/* Icon Text */}
      <text
        x={x}
        y={y}
        textAnchor="middle"
        dominantBaseline="middle"
        fill={isHovered ? "var(--color-orange)" : "var(--color-ink-2)"}
        className="text-[10px] font-mono select-none"
      >
        {icon}
      </text>

      {/* Label */}
      <text
        x={x}
        y={y + 40}
        textAnchor="middle"
        fill={isHovered ? "var(--color-ink)" : "var(--color-ink-3)"}
        className="text-[11px] font-medium font-sans select-none"
      >
        {label}
      </text>
    </motion.g>
  );
};

const Connection = ({ x1, y1, x2, y2, active }: { x1: number, y1: number, x2: number, y2: number, active: boolean }) => (
  <motion.line
    x1={x1}
    y1={y1}
    x2={x2}
    y2={y2}
    animate={{
      stroke: active ? "var(--color-orange)" : "var(--color-border)",
      strokeWidth: active ? 2 : 1,
      opacity: active ? 0.8 : 0.3,
    }}
    strokeDasharray={active ? "none" : "4 4"}
  />
);

const NODES = [
  {
    id: "nextjs",
    label: "Frontend (Next.js)",
    x: 420,
    y: 120,
    icon: "JS",
    href: "/docs/scaffold/nextjs",
    description: "Modern UI with App Router, TypeScript, and optimized assets."
  },
  {
    id: "rust",
    label: "Backend (Axum)",
    x: 180,
    y: 120,
    icon: "RS",
    href: "/docs/scaffold/rust",
    description: "High-performance async server with typed safety and structured logging."
  },
  {
    id: "database",
    label: "Database (SQLx/SeaORM)",
    x: 180,
    y: 240,
    icon: "DB",
    href: "/docs/configuration/database",
    description: "Type-safe database access with automatic migrations and connection pooling."
  },
  {
    id: "auth",
    label: "Auth (JWT)",
    x: 180,
    y: 30,
    icon: "AU",
    href: "/docs/configuration/authentication",
    description: "Secure authentication middleware with JWT validation and route protection."
  }
];

export default function BlueprintNavigator() {
  const [hoveredNode, setHoveredNode] = useState<string | null>(null);

  const activeDescription = NODES.find(n => n.id === hoveredNode)?.description;

  return (
    <div className="my-12 p-6 machined-card overflow-hidden">
      <div className="flex flex-col md:flex-row items-center gap-8">
        {/* Diagram Area */}
        <div className="relative w-full max-w-[500px] aspect-[5/3]">
          <svg
            viewBox="0 0 600 300"
            className="w-full h-full"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            {/* Background Grid */}
            <defs>
              <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
                <path d="M 40 0 L 0 0 0 40" fill="none" stroke="var(--color-border)" strokeWidth="0.5" opacity="0.1"/>
              </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#grid)" />

            {/* Connections */}
            <Connection x1={180} y1={120} x2={420} y2={120} active={hoveredNode === "rust" || hoveredNode === "nextjs"} />
            <Connection x1={180} y1={120} x2={180} y2={240} active={hoveredNode === "rust" || hoveredNode === "database"} />
            <Connection x1={180} y1={120} x2={180} y2={30} active={hoveredNode === "rust" || hoveredNode === "auth"} />

            {/* Nodes */}
            {NODES.map((node) => (
              <Node
                key={node.id}
                {...node}
                isHovered={hoveredNode === node.id}
                onHover={setHoveredNode}
              />
            ))}
          </svg>
        </div>

        {/* Info Panel */}
        <div className="flex-1 min-h-[140px] flex flex-col justify-center border-l border-border pl-8">
          <AnimatePresence mode="wait">
            {hoveredNode ? (
              <motion.div
                key={hoveredNode}
                initial={{ opacity: 0, x: 10 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -10 }}
                transition={{ duration: 0.2 }}
              >
                <h3 className="text-orange font-mono text-sm mb-2 uppercase tracking-wider">
                  {NODES.find(n => n.id === hoveredNode)?.label}
                </h3>
                <p className="text-ink-2 text-sm leading-relaxed mb-4">
                  {activeDescription}
                </p>
                <a 
                  href={NODES.find(n => n.id === hoveredNode)?.href}
                  className="text-ink text-xs font-semibold underline decoration-orange/30 underline-offset-4 hover:decoration-orange transition-all"
                >
                  View documentation →
                </a>
              </motion.div>
            ) : (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="text-ink-3 text-sm italic"
              >
                Hover over a component in the blueprint to explore the architecture...
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
