import sharp from 'sharp';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { readFileSync, mkdirSync } from 'fs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const publicDir = join(__dirname, '..', 'public');

// Favicon path as inline SVG for embedding
const faviconSvg = readFileSync(join(publicDir, 'favicon.svg'), 'utf8');
const faviconB64 = Buffer.from(faviconSvg).toString('base64');

const W = 1200;
const H = 630;

// Grid lines: 40px spacing, phosphor green at 4% opacity
const gridLines = [];
for (let x = 0; x <= W; x += 40) {
  gridLines.push(`<line x1="${x}" y1="0" x2="${x}" y2="${H}" stroke="#00ff41" stroke-width="0.5" opacity="0.08"/>`);
}
for (let y = 0; y <= H; y += 40) {
  gridLines.push(`<line x1="0" y1="${y}" x2="${W}" y2="${y}" stroke="#00ff41" stroke-width="0.5" opacity="0.08"/>`);
}

const svg = `
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}">
  <defs>
    <!-- Radial glows -->
    <radialGradient id="glow-orange" cx="15%" cy="85%" r="40%">
      <stop offset="0%" stop-color="#f44336" stop-opacity="0.18"/>
      <stop offset="100%" stop-color="#f44336" stop-opacity="0"/>
    </radialGradient>
    <radialGradient id="glow-green" cx="85%" cy="20%" r="35%">
      <stop offset="0%" stop-color="#00ff41" stop-opacity="0.12"/>
      <stop offset="100%" stop-color="#00ff41" stop-opacity="0"/>
    </radialGradient>
    <!-- Scanline texture -->
    <pattern id="scanlines" x="0" y="0" width="100%" height="2" patternUnits="userSpaceOnUse">
      <rect x="0" y="0" width="100%" height="1" fill="black" opacity="0.06"/>
    </pattern>
  </defs>

  <!-- Background -->
  <rect width="${W}" height="${H}" fill="#0a0a0a"/>

  <!-- Grid -->
  ${gridLines.join('\n  ')}

  <!-- Ambient glows -->
  <rect width="${W}" height="${H}" fill="url(#glow-orange)"/>
  <rect width="${W}" height="${H}" fill="url(#glow-green)"/>

  <!-- Scanlines overlay -->
  <rect width="${W}" height="${H}" fill="url(#scanlines)"/>

  <!-- Left accent bar -->
  <rect x="72" y="180" width="4" height="270" fill="#f44336" opacity="0.9"/>

  <!-- Logo mark (favicon) — 72×72 at top-left text block -->
  <image href="data:image/svg+xml;base64,${faviconB64}" x="92" y="183" width="60" height="60"
    style="filter: invert(1) sepia(1) saturate(5) hue-rotate(310deg) brightness(1.1)"/>

  <!-- Main title -->
  <text
    x="92"
    y="330"
    font-family="'JetBrains Mono', 'Courier New', Courier, monospace"
    font-size="72"
    font-weight="700"
    letter-spacing="-1"
    fill="#f44336"
    filter="url(#orange-glow)"
  >create-ruxum-app</text>

  <!-- Subtitle -->
  <text
    x="96"
    y="393"
    font-family="'JetBrains Mono', 'Courier New', Courier, monospace"
    font-size="26"
    font-weight="400"
    fill="#00ff41"
    opacity="0.9"
    letter-spacing="0.5"
  >Scaffold Rust + Next.js at terminal velocity.</text>

  <!-- Bottom terminal prompt line -->
  <text
    x="92"
    y="490"
    font-family="'JetBrains Mono', 'Courier New', Courier, monospace"
    font-size="20"
    fill="#ffffff"
    opacity="0.25"
    letter-spacing="0.3"
  >$ npx create-ruxum-app@latest</text>

  <!-- Blinking cursor rect -->
  <rect x="456" y="470" width="12" height="22" fill="#00ff41" opacity="0.5">
    <animate attributeName="opacity" values="0.5;0;0.5" dur="1.2s" repeatCount="indefinite"/>
  </rect>

  <!-- Corner bracket TL -->
  <path d="M40 40 L40 90 M40 40 L90 40" stroke="#f44336" stroke-width="2" fill="none" opacity="0.4"/>
  <!-- Corner bracket BR -->
  <path d="M${W-40} ${H-40} L${W-40} ${H-90} M${W-40} ${H-40} L${W-90} ${H-40}" stroke="#00ff41" stroke-width="2" fill="none" opacity="0.4"/>

  <!-- Top-right badge: "Rust + Next.js" -->
  <rect x="${W-260}" y="42" width="220" height="38" rx="3" fill="#1a1a1a" stroke="#f44336" stroke-width="1" opacity="0.8"/>
  <text
    x="${W-150}"
    y="67"
    font-family="'JetBrains Mono', 'Courier New', Courier, monospace"
    font-size="15"
    fill="#f44336"
    text-anchor="middle"
    letter-spacing="1.5"
  >RUST · AXUM · NEXT.JS</text>
</svg>`.trim();

mkdirSync(publicDir, { recursive: true });

await sharp(Buffer.from(svg))
  .resize(W, H)
  .png({ compressionLevel: 9 })
  .toFile(join(publicDir, 'og-image.png'));

console.log('✓ Generated www/public/og-image.png (1200×630)');
