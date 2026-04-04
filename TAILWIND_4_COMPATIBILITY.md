# Tailwind CSS 4 Compatibility & Deployment Guide

## Overview
This project uses Tailwind CSS 4.2.2 with the new Vite integration. This document ensures compatibility across browsers and deployment platforms.

## Browser Support Matrix

### ✅ Fully Supported
| Browser | Minimum Version | Notes |
|---------|---|---|
| Chrome/Edge | 88+ | Full CSS custom properties support |
| Firefox | 85+ | CSS custom properties, `color-mix()` |
| Safari | 15.4+ | ⚠️ Some CSS features may be limited |
| iOS Safari | 15.4+ | |
| Chrome Android | Latest | |

### ❌ Not Supported
- **Internet Explorer 11** - No CSS custom properties or modern CSS
- **Safari < 15.4** - Missing CSS custom properties and `color-mix()`
- **Firefox < 85** - Incomplete CSS custom properties support

## Modern CSS Features Used

### In `src/styles/global.css`

#### 1. CSS Custom Properties (`@theme`)
```css
@theme {
  --color-canvas: #0f0e0d;
  --color-surface: #161514;
  --animate-forge-glow: forge-glow 10s ease-in-out infinite;
}
```
- Requires: CSS Variables (CSS 3)
- Browser support: 95%+ of users

#### 2. CSS Functions
- `color-mix()` - Tailwind 4 dynamic colors
- `linear-gradient()`, `radial-gradient()` - Standard CSS 3
- `clamp()` - Modern responsive sizing

#### 3. Advanced Selectors
- `::-webkit-scrollbar` - Webkit browsers
- `::-moz-*` - Firefox specific

#### 4. Filter & Transform
- `filter: blur()`, `brightness()` - CSS Filters Module Level 1
- `transform: translateX()`, `scale()` - CSS Transforms

## Development Environment

### Required Versions
```json
{
  "astro": "^6.0.0",
  "@tailwindcss/vite": "^4.2.2",
  "tailwindcss": "^4.2.2"
}
```

### Setup Checklist
- [ ] `npm install` after package.json updates
- [ ] Clear `.astro` build cache if styles don't update
- [ ] Verify `tailwindcss` is installed (not just `@tailwindcss/vite`)

## Vercel Deployment

### Build Configuration
The project uses Astro's static output (`output: 'static'`), which is optimal for Vercel:

```javascript
// astro.config.mjs
export default defineConfig({
  output: 'static',
  vite: {
    plugins: [tailwindcss()],
  },
});
```

### Verified on Vercel
- ✅ Astro 6 with Tailwind 4 Vite plugin
- ✅ Static site generation
- ✅ No server-side rendering issues
- ✅ CSS optimization via Tailwind minification

### Potential Issues & Solutions

#### Issue: CSS Not Applied in Production
- **Cause**: Stale node_modules or missing `tailwindcss` package
- **Fix**: 
  - Trigger fresh deployment
  - Delete `.vercel` cache
  - Ensure both `@tailwindcss/vite` and `tailwindcss` are in dependencies

#### Issue: Build Timeout
- **Cause**: Large CSS file generation
- **Fix**: 
  - Limit unused CSS with proper Tailwind content config
  - Use dynamic class generation cautiously

#### Issue: Fonts Not Loading
- **Cause**: Font preload paths incorrect
- **Fix**: Verify `BaseLayout.astro` font paths are correct

## PostCSS Configuration

Tailwind 4 does NOT require PostCSS configuration when using the Vite plugin. The plugin handles:
- CSS processing
- Tailwind compilation
- Autoprefixing
- Minification

No `postcss.config.js` or `tailwind.config.js` needed (unless customization required).

## Testing Browser Compatibility

### Local Testing
```bash
# Test in Safari on macOS
open -a Safari http://localhost:3000

# Test in Firefox
firefox http://localhost:3000

# Test in Chrome DevTools device emulation
# Chrome DevTools > Ctrl+Shift+M > Select device
```

### Remote Testing (BrowserStack/LambdaTest)
- Test iOS Safari 15.4+ on real devices
- Verify custom CSS variables render correctly
- Check gradient/filter effects on older Safari versions

### Automated Testing
- [ ] Add CI/CD checks for CSS validity
- [ ] Monitor browser stats via analytics
- [ ] Set minimum browser version policy

## Migration from Older Tailwind Versions

If this was updated from Tailwind 3:
- ⚠️ Removed PostCSS plugin requirement
- ⚠️ `@apply` directive behavior changed
- ⚠️ New color functions (`color-mix()`, `color()`)
- ⚠️ CSS custom properties now primary for theming

### Breaking Changes to Watch
1. Custom config files may need updates
2. JIT mode is now default (content scanning)
3. Color opacity syntax changed
4. Some deprecated utilities removed

## Performance Impact

### CSS Bundle Size
- Tailwind 4 with Vite: ~40-60KB (minified)
- With unused CSS purge: ~10-20KB

### Build Time
- Astro with Tailwind 4 Vite: ~2-5 seconds
- No separate PostCSS compilation step

## References
- [Tailwind CSS 4 Docs](https://tailwindcss.com/docs)
- [Astro + Tailwind Integration](https://docs.astro.build/en/guides/styling/#tailwind)
- [Browser Compatibility for CSS Custom Properties](https://caniuse.com/css-variables)
