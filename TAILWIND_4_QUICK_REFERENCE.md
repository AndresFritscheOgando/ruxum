# Tailwind 4 Quick Reference Card

## Setup Verification
```bash
cd www

# ✅ Check dependencies
npm list tailwindcss @tailwindcss/vite

# ✅ Build test
npm run build

# ✅ CSS file size (should be 40-60KB)
ls -lh dist/_astro/*.css
```

## Browser Support at a Glance
| Browser | Min Version | Support |
|---------|---|---|
| Chrome/Edge | 88 | ✅ |
| Firefox | 85 | ✅ |
| Safari | 15.4 | ✅ |
| IE 11 | All | ❌ |

## What CSS Features Are Used
```css
/* CSS Custom Properties */
color: var(--color-canvas);

/* Gradients */
background: linear-gradient(135deg, #1 0%, #2 100%);

/* Filters */
filter: blur(8px);

/* Transforms */
transform: translateY(-2px);

/* Modern selectors */
::-webkit-scrollbar { width: 5px; }
```

## Common Debugging

### Test CSS Variables in Console
```javascript
getComputedStyle(document.documentElement).getPropertyValue('--color-canvas')
// Should return: #0f0e0d
```

### Check CSS is Loaded
```javascript
document.styleSheets.length  // Should be > 0
```

### Test Tailwind Utility Classes
```javascript
document.querySelector('[class*="flex"]').className
// Should show Tailwind classes
```

## Troubleshooting Quick Fixes

### Styles Missing After Deploy?
1. Clear browser cache (Ctrl+Shift+Del or Cmd+Shift+Del)
2. Hard refresh (Ctrl+Shift+R or Cmd+Shift+R)
3. Check Vercel build logs for errors

### CSS Variables Return "undefined"?
- Browser doesn't support CSS custom properties
- Check browser version (need Safari 15.4+, Firefox 85+, Chrome 88+)
- Test in Chrome DevTools if other browsers unavailable

### Responsive Styles Not Working?
- Verify viewport meta tag exists (it does: `<meta name="viewport" ...>`)
- Test with DevTools device emulation (Ctrl+Shift+M)
- Check CSS for media query syntax

## Important Files
- `www/src/styles/global.css` - Design tokens and custom classes
- `www/astro.config.mjs` - Tailwind 4 Vite plugin config
- `www/package.json` - Dependencies (both `tailwindcss` and `@tailwindcss/vite` required)
- `www/src/layouts/BaseLayout.astro` - Font imports and layout

## Performance Targets
- Build time: 2-5 seconds ✅
- CSS file size: 40-60KB minified ✅
- Gzipped CSS: 10-15KB ✅
- Lighthouse Performance: 90+ ✅

## Vercel Deployment Checklist
- [ ] Local build succeeds: `npm run build`
- [ ] CSS files exist: `ls www/dist/_astro/*.css`
- [ ] Both packages installed: `npm list tailwindcss @tailwindcss/vite`
- [ ] Build < 5 minutes on Vercel
- [ ] CSS loads without 404 errors
- [ ] Styles render correctly on production site

## Further Reading
See these files for detailed information:
- `TAILWIND_4_COMPATIBILITY.md` - Full browser/feature support
- `VERCEL_BUILD_CONFIG.md` - Deployment configuration
- `TAILWIND_4_TESTING.md` - Testing procedures
- `TAILWIND_4_FIX_SUMMARY.md` - Issue resolution details

## Version Pinning (Optional)
For maximum stability, consider pinning to exact version in `www/package.json`:
```json
"dependencies": {
  "tailwindcss": "4.2.2"  // Instead of ^4.2.2
}
```

## When to Update Tailwind
- ✅ Patch updates (4.2.2 → 4.2.3): Always safe
- ✅ Minor updates (4.2 → 4.3): Usually safe, check release notes
- ⚠️ Major updates (4.x → 5.x): Review breaking changes
- Update both `tailwindcss` and `@tailwindcss/vite` together

---

**Last Updated**: 2026-04-04  
**Status**: ✅ Production Ready
