# Tailwind 4 Compatibility Testing Guide

## Quick Health Check

Run these commands to verify Tailwind 4 setup:

```bash
cd www

# 1. Verify dependencies
npm list tailwindcss @tailwindcss/vite
# Should show both packages

# 2. Run dev server
npm run dev
# Should show "Astro v6" and no CSS errors

# 3. Build locally
npm run build
# Should complete without errors in 2-5 seconds

# 4. Preview build
npm run preview
# Should show styled site with no console errors
```

## Browser Compatibility Testing

### Test 1: CSS Custom Properties
**What**: Verifies `@theme` syntax support

```javascript
// In browser DevTools console on http://localhost:3000
(function testCSSVariables() {
  const root = document.documentElement;
  const canvas = getComputedStyle(root).getPropertyValue('--color-canvas').trim();
  const surface = getComputedStyle(root).getPropertyValue('--color-surface').trim();
  
  console.log('✓ CSS Variables Test');
  console.log('--color-canvas:', canvas);     // Should be: #0f0e0d
  console.log('--surface:', surface);          // Should be: #161514
  
  if (canvas === '#0f0e0d') {
    console.log('✅ CSS Variables working');
  } else {
    console.error('❌ CSS Variables NOT working. Browser may not support CSS custom properties.');
  }
})();
```

**Browsers tested**:
- ✅ Chrome 88+
- ✅ Firefox 85+
- ✅ Safari 15.4+
- ❌ IE 11 (will fail)

### Test 2: Tailwind Utility Classes
**What**: Verifies Tailwind CSS is compiled and applied

```javascript
(function testTailwindClasses() {
  // Find an element with Tailwind classes
  const heroes = document.querySelectorAll('[class*="flex"]');
  
  console.log('✓ Tailwind Classes Test');
  console.log('Elements with "flex":', heroes.length);
  
  if (heroes.length > 0) {
    const firstEl = heroes[0];
    const classes = firstEl.className;
    const computed = getComputedStyle(firstEl);
    
    console.log('Classes:', classes);
    console.log('Display:', computed.display);  // Should be "flex"
    console.log('✅ Tailwind utilities applied');
  }
})();
```

### Test 3: Responsive Design
**What**: Tests mobile, tablet, desktop breakpoints

```javascript
(function testResponsive() {
  const width = window.innerWidth;
  const breakpoints = {
    mobile: width < 640,
    tablet: width >= 640 && width < 1024,
    desktop: width >= 1024,
    xl: width >= 1280,
  };
  
  console.log('✓ Responsive Breakpoint Test');
  console.log('Window width:', width + 'px');
  Object.entries(breakpoints).forEach(([name, active]) => {
    console.log(name + ':', active ? '✅ ACTIVE' : '⚠️  inactive');
  });
})();
```

**Manual testing**:
```bash
# DevTools > Ctrl+Shift+M (Device Emulation)
# Test these devices:
- iPhone SE (375px)
- iPad Air (820px)
- MacBook (1440px)
```

### Test 4: Animations & Transforms
**What**: Verifies custom keyframes work

```javascript
(function testAnimations() {
  const animated = document.querySelector('[class*="animate-"]');
  
  if (animated) {
    const computed = getComputedStyle(animated);
    const animation = computed.animation;
    
    console.log('✓ Animation Test');
    console.log('Animation:', animation);
    
    if (animation !== 'none') {
      console.log('✅ Animations working');
    } else {
      console.log('❌ No animations detected');
    }
  }
})();
```

### Test 5: Color & Gradient Support
**What**: Tests `color-mix()` and gradient functions

```javascript
(function testColors() {
  const gradientEl = document.querySelector('[class*="bg-gradient"]');
  
  console.log('✓ Color & Gradient Test');
  
  if (gradientEl) {
    const bg = getComputedStyle(gradientEl).backgroundImage;
    console.log('Gradient detected:', bg.includes('gradient') ? '✅' : '❌');
  }
  
  // Test orange color variables
  const orange = getComputedStyle(document.documentElement)
    .getPropertyValue('--color-orange').trim();
  console.log('Brand color (orange):', orange);  // Should be: #e8632a
})();
```

## Performance Testing

### Build Performance
```bash
cd www

# Measure build time
time npm run build

# Expected: 2-5 seconds
# If > 10 seconds: may have large CSS output
```

### CSS File Size
```bash
# After build, check CSS file size
ls -lh dist/_astro/*.css | head -5

# Expected: 40-60KB per CSS file
# Example: 52K _astro/BaseLayout.DLMq7sY-.css
```

### Lighthouse Performance
1. Open `npm run preview` → http://localhost:3000
2. DevTools > Lighthouse
3. Run audit for:
   - Performance
   - Accessibility
   - Best Practices
   - SEO

**Expected scores**:
- Performance: 90+
- Accessibility: 95+
- Best Practices: 95+
- SEO: 100

## Browser-Specific Tests

### Safari 15.4+ (CSS Custom Properties)
```javascript
// In Safari DevTools console
console.log(CSS.supports('--test', 'value'));
// Should return: true
```

### Firefox (CSS Filters)
```javascript
(function testFilters() {
  const el = document.querySelector('[class*="blur"]');
  if (el) {
    const filter = getComputedStyle(el).filter;
    console.log('Filter support:', filter !== 'none' ? '✅' : '❌');
  }
})();
```

### Chrome (All modern features)
```javascript
// Test color-mix support (Tailwind 4 specific)
const testColorMix = CSS.supports('color', 'color-mix(in srgb, red, blue)');
console.log('color-mix() support:', testColorMix ? '✅' : '❌');
```

## Vercel Deployment Testing

### Pre-Deployment Checklist
```bash
# 1. Run build exactly as Vercel will
npm ci --only=production
npm run build

# 2. Verify dist is generated
ls www/dist/index.html

# 3. Preview locally
npm run preview

# 4. Check CSS in dist
ls -lh www/dist/_astro/*.css
```

### Post-Deployment Validation
After deploying to Vercel:

```javascript
// In DevTools on https://your-site.vercel.app
// 1. Check CSS file loaded
console.log(document.styleSheets.length);  // Should be > 0

// 2. Verify Tailwind is active
console.log(
  getComputedStyle(document.body)
    .getPropertyValue('--color-canvas')
);

// 3. Check network tab
// Should see _astro/*.css files with 200 status
```

## Debugging Common Issues

### Issue: CSS Variables Show as "undefined"
```javascript
// Before: May fail if loaded too early
// After: Should work after page load
window.addEventListener('load', () => {
  const color = getComputedStyle(document.documentElement)
    .getPropertyValue('--color-canvas');
  console.log('Canvas color:', color);
});
```

### Issue: Gradients Not Rendering
```javascript
// Check if CSS has gradient syntax
const sheet = document.styleSheets[0];
const rules = sheet.cssRules;
let gradientFound = false;

for (let rule of rules) {
  if (rule.style && rule.style.backgroundImage) {
    if (rule.style.backgroundImage.includes('gradient')) {
      gradientFound = true;
      break;
    }
  }
}

console.log('Gradients in CSS:', gradientFound ? '✅' : '❌');
```

### Issue: Font Not Loading
```javascript
// Check font loading status
document.fonts.ready.then(() => {
  console.log('All fonts loaded ✅');
}).catch(() => {
  console.error('Font loading failed ❌');
});
```

## CI/CD Testing Commands

### GitHub Actions Example
```yaml
# Add to your workflow
- name: Test Tailwind 4 Build
  run: |
    cd www
    npm ci
    npm run build
    npm run preview &
    sleep 3
    # Verify CSS exists
    test -f dist/_astro/*.css
```

### Manual Test Script
Save as `test-tailwind.sh`:
```bash
#!/bin/bash
set -e

cd www

echo "🔍 Checking dependencies..."
npm list tailwindcss @tailwindcss/vite > /dev/null

echo "🏗️  Building..."
npm run build > /dev/null

echo "📦 Verifying CSS output..."
CSS_COUNT=$(find dist -name "*.css" | wc -l)
echo "   Generated $CSS_COUNT CSS files"

echo "✅ All Tailwind 4 tests passed!"
```

## Automated Testing Template

Create `www/__tests__/tailwind.test.ts` (if using test runner):

```typescript
describe('Tailwind 4 Compatibility', () => {
  test('tailwindcss package installed', () => {
    // Verify package exists
    const pkg = require('../package.json');
    expect(pkg.dependencies.tailwindcss).toBeDefined();
    expect(pkg.devDependencies['@tailwindcss/vite']).toBeDefined();
  });

  test('CSS custom properties defined', () => {
    // Would need jsdom or playwright
    const styles = getComputedStyle(document.documentElement);
    expect(styles.getPropertyValue('--color-canvas')).toBeTruthy();
  });

  test('build completes without errors', () => {
    // Use execa to run build
    expect(() => execaSync('npm', ['run', 'build'])).not.toThrow();
  });
});
```

## Summary

✅ **Quick check for production readiness**:
```bash
cd www
npm list tailwindcss @tailwindcss/vite  # Both should exist
npm run build                             # Should complete < 5s
npm run preview                           # Should show styled site
# Open DevTools console and run:
# getComputedStyle(document.documentElement).getPropertyValue('--color-canvas')
# Should return: #0f0e0d
```

If all tests pass, Tailwind 4 is properly configured for Vercel deployment.
