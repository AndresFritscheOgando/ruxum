# EC2 Quick Fix - Get Your Site Running Now

## Current Problem
You're running `npm run dev` on EC2, which starts Vite dev server with `allowedHosts` security check.

## Immediate Fix (5 minutes)

### 1. SSH into your EC2 instance
```bash
ssh -i your-key.pem ec2-user@your-ec2-ip
```

### 2. Kill the dev server
```bash
# Find the process
ps aux | grep "astro dev"

# Kill it
pkill -f "astro dev"
# or
kill -9 <PID>
```

### 3. Build the site
```bash
cd /path/to/ruxum
npm --prefix www run build
```

### 4. Install Nginx
```bash
sudo yum install -y nginx
sudo systemctl start nginx
sudo systemctl enable nginx
```

### 5. Create Nginx config
```bash
sudo tee /etc/nginx/conf.d/ruxum.conf > /dev/null << 'EOF'
server {
    listen 80;
    server_name ruxum.dev www.ruxum.dev _;
    
    root /path/to/ruxum/www/dist;
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 30d;
    }
}
EOF
```

### 6. Test and restart Nginx
```bash
sudo nginx -t
sudo systemctl restart nginx
```

### 7. Verify it's working
```bash
curl http://ruxum.dev
# Should return HTML, not Vite error
```

---

## Why This Fixes It

| Issue | Root Cause | Solution |
|-------|-----------|----------|
| "Host not allowed" | Running Vite dev server | Build static files, serve with Nginx |
| Port conflicts | Dev server on port 3000/5173 | Nginx on port 80 |
| Domain restrictions | Vite security check | Static server doesn't have this check |

---

## What You Should NOT Do

❌ Don't add `allowedHosts` to bypass the issue  
❌ Don't run `npm run dev` in production  
❌ Don't use `--allow-all` flags  

These are security features for development, not production.

---

## Verify the Fix

After setup, check:

```bash
# 1. Nginx is running
sudo systemctl status nginx

# 2. Site is accessible
curl -I http://ruxum.dev
# Should return 200, not Vite error

# 3. Files are being served
curl http://ruxum.dev | head -20
# Should show HTML, not error message

# 4. CSS and JS load
curl http://ruxum.dev/_astro/*.css | head -5
# Should show CSS content
```

---

## Next: Add HTTPS

```bash
# Install Certbot
sudo yum install -y certbot python3-certbot-nginx

# Get free certificate
sudo certbot certonly --nginx -d ruxum.dev -d www.ruxum.dev

# Update Nginx to use HTTPS
# Edit /etc/nginx/conf.d/ruxum.conf and add SSL settings
# See EC2_DEPLOYMENT_GUIDE.md for full config
```

---

## If You Have Issues

### Issue: Permission Denied on dist folder
```bash
sudo chown -R nginx:nginx /path/to/ruxum/www/dist
sudo chmod -R 755 /path/to/ruxum/www/dist
```

### Issue: Port 80 already in use
```bash
sudo lsof -i :80
sudo systemctl stop <service-using-80>
```

### Issue: DNS not resolving
```bash
# Ensure your domain points to EC2 instance IP
# Check in route53 or your DNS provider
nslookup ruxum.dev
```

### Issue: Still seeing Vite error
```bash
# Make sure dev server is killed
ps aux | grep node
ps aux | grep astro
# Kill any remaining processes
```

---

## Support

See `EC2_DEPLOYMENT_GUIDE.md` for complete setup with:
- PM2 process management
- Auto-restart configuration
- Monitoring and logs
- Backup strategy
- Security hardening

---

**Time to fix**: 5-10 minutes  
**Recommended**: Do this now, then read full guide later
