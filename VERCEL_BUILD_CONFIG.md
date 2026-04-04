# Vercel Build Configuration for Tailwind 4

## Build Process Overview

### Default Vercel Settings for Astro
```
Framework: Astro
Build Command: npm run build
Output Directory: dist
Node Version: 18.x (default) → Recommend: 20.x or 22.x
```

## Recommended Configuration

### 1. Create `vercel.json` (Optional but Recommended)
```json
{
  "buildCommand": "npm run build",
  "outputDirectory": "www/dist",
  "framework": "astro",
  "nodeVersion": "20.x",
  "env": {
    "NODE_OPTIONS": "--max-old-space-size=3072"
  }
}
```

Place in project root: `vercel.json`

### 2. Environment Variables
Set in Vercel Dashboard → Settings → Environment Variables:

```
NODE_ENV=production
SKIP_ENV_VALIDATION=true (if using env validation)
```

### 3. Build Command
Ensure `www/package.json` has:
```json
{
  "scripts": {
    "build": "astro build",
    "dev": "astro dev",
    "preview": "astro preview"
  }
}
```

## Tailwind 4 + Vite Specific Considerations

### ✅ What Works Well
- Vite is optimized for Vercel's build environment
- SSG (Static Site Generation) = no runtime overhead
- Fast builds with minimal caching issues
- No separate PostCSS compilation step

### ⚠️ Potential Issues

#### Issue: Tailwind CSS not generated in production
**Symptoms**: Deployed site shows no styles

**Diagnosis**:
1. Check Vercel build logs for errors:
   ```
   npm ERR! Missing peer dependencies:
   npm ERR! tailwindcss@^4.2.2
   ```

**Prevention**:
- ✅ Explicit `tailwindcss: ^4.2.2` in dependencies (just added)
- ✅ Both `@tailwindcss/vite` and `tailwindcss` required
- ✅ `npm ci` (not `npm install`) for reproducible builds

#### Issue: Build timeout (>15 minutes)
**Cause**: CSS file too large from unused styles

**Solution**:
```javascript
// astro.config.mjs - Verify content paths
export default defineConfig({
  vite: {
    plugins: [tailwindcss()],
  },
});
```

Tailwind 4 should auto-detect content paths, but verify:
- All `.astro` files are scanned
- All `.tsx` files are scanned
- No dynamic class strings (use CSS variables instead)

#### Issue: Font loading fails on production
**Cause**: Relative paths in font preloads

**Status**: ✅ Verified in `www/src/layouts/BaseLayout.astro:22-35`
Uses `?url` imports which Vite handles correctly.

## Deployment Verification Checklist

### Before Deploying to Production

- [ ] Run `npm install` locally (not just `npm ci`)
- [ ] Run `npm run build` successfully
- [ ] Verify `npm run preview` shows correct styles
- [ ] Check that both packages exist:
  ```bash
  npm list tailwindcss @tailwindcss/vite
  ```
- [ ] Test CSS in multiple browsers (Chrome, Firefox, Safari)
- [ ] Verify custom CSS variables are rendered:
  ```javascript
  // In browser console
  console.log(getComputedStyle(document.body).getPropertyValue('--color-canvas'));
  // Should output: #0f0e0d
  ```

### After Deploying

- [ ] Check Vercel build logs for warnings/errors
- [ ] Inspect deployed site CSS (DevTools > Network > CSS files)
- [ ] Verify CSS file size (should be ~40-60KB minified)
- [ ] Test on Safari 15+ (CSS custom properties support)
- [ ] Test responsive styles with device emulation
- [ ] Monitor Core Web Vitals in Vercel Analytics

## Rollback Plan

If CSS breaks on deployment:

1. **Immediate**: Trigger a new deployment from git (forces rebuild)
2. **Short-term**: Check git history for package.json changes
3. **Long-term**: Consider pinning `tailwindcss` to exact version:
   ```json
   "tailwindcss": "4.2.2"  // Instead of ^4.2.2
   ```

## Performance Metrics

### Expected Build Times
- Full build: 1-3 minutes
- Cache hit: 30-60 seconds
- CSS generation: < 10 seconds

### Expected CSS Output
- CSS file size: ~40-60KB (minified, gzipped ~10-15KB)
- Can monitor in Vercel Analytics

## Debugging on Vercel

### Enable Verbose Build Logs
1. Vercel Dashboard → Project Settings → Build & Development
2. Check "Verbose logging"
3. Trigger new deployment
4. Review logs for `@tailwindcss/vite` initialization

### Common Build Log Patterns
```
✓ Installed 1234 packages
✓ Building with Astro
✓ Generating CSS with Tailwind 4
✓ Minifying assets
✓ Generated 45 pages in 2.3s
```

### Check CSS Generation
```bash
# After preview build, CSS should exist:
ls www/dist/_astro/*.css
# Look for generated CSS files with content
```

## Future Updates

### Tailwind 4.x Updates
- Minor version bumps (4.2 → 4.3) are safe
- Check release notes for deprecations
- Update both `@tailwindcss/vite` AND `tailwindcss` together

### Astro Updates  
- Astro 6+ will continue supporting Tailwind 4 Vite plugin
- Monitor Astro release notes for integration changes

## References
- [Vercel Astro Deployment](https://vercel.com/docs/frameworks/astro)
- [Tailwind 4 with Vite](https://tailwindcss.com/docs/guides/vite)
- [Astro Vercel Adapter](https://docs.astro.build/en/guides/deploy/vercel/)
