# AWS EC2 Deployment Guide for Astro Static Site

## Problem
Running `npm run dev` (Vite dev server) on EC2 causes "host not allowed" errors because Vite blocks requests from non-localhost domains.

## Solution
Deploy as a **static site** using a proper web server, not the development server.

---

## Option 1: Nginx + Static Files (Recommended)

### 1. SSH into your EC2 instance
```bash
ssh -i your-key.pem ec2-user@your-ec2-ip
```

### 2. Install dependencies
```bash
# Update system
sudo yum update -y

# Install Node.js (22.x recommended)
curl -fsSL https://rpm.nodesource.com/setup_22.x | sudo bash -
sudo yum install -y nodejs

# Install Nginx
sudo yum install -y nginx

# Start and enable Nginx
sudo systemctl start nginx
sudo systemctl enable nginx
```

### 3. Clone and build your project
```bash
cd /home/ec2-user  # or your desired directory

# Clone repo
git clone https://github.com/AndresFritscheOgando/ruxum.git
cd ruxum

# Install dependencies
npm install
cd www && npm install && cd ..

# Build the static site
npm --prefix www run build

# The static files are now in: www/dist/
```

### 4. Configure Nginx
Create `/etc/nginx/conf.d/ruxum.conf`:

```nginx
server {
    listen 80;
    server_name ruxum.dev www.ruxum.dev _;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Serve static files
    root /home/ec2-user/ruxum/www/dist;
    index index.html;

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    # SPA routing: serve index.html for all routes
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Deny access to sensitive files
    location ~ /\. {
        deny all;
    }
}
```

### 5. Test and restart Nginx
```bash
# Test config
sudo nginx -t

# If OK, restart
sudo systemctl restart nginx

# Check status
sudo systemctl status nginx
```

### 6. Set up HTTPS with Let's Encrypt (Important!)
```bash
# Install Certbot
sudo yum install -y certbot python3-certbot-nginx

# Get certificate
sudo certbot certonly --nginx -d ruxum.dev -d www.ruxum.dev

# Auto-renewal
sudo systemctl enable certbot-renewal.timer
sudo systemctl start certbot-renewal.timer
```

Update Nginx config to redirect HTTP → HTTPS:

```nginx
server {
    listen 80;
    server_name ruxum.dev www.ruxum.dev;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name ruxum.dev www.ruxum.dev;

    ssl_certificate /etc/letsencrypt/live/ruxum.dev/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/ruxum.dev/privkey.pem;

    # ... rest of config from above ...
}
```

---

## Option 2: Node.js Server (Alternative)

If you prefer to run Node.js directly:

### 1. Install
```bash
npm install --global serve
```

### 2. Serve static files
```bash
cd /home/ec2-user/ruxum/www
serve -s dist -p 3000 -c dist/index.html
```

### 3. Use PM2 for process management
```bash
# Install PM2
sudo npm install -g pm2

# Create ecosystem.config.js
cat > ecosystem.config.js << 'EOF'
module.exports = {
  apps: [{
    name: 'ruxum-site',
    script: 'serve',
    args: '-s dist -p 3000 -c dist/index.html',
    cwd: '/home/ec2-user/ruxum/www',
    env: {
      NODE_ENV: 'production'
    },
    restart_delay: 4000,
    max_memory_restart: '1G'
  }]
};
EOF

# Start with PM2
pm2 start ecosystem.config.js
pm2 save
pm2 startup

# Verify it's running
pm2 list
```

### 4. Put Nginx in front as reverse proxy
```nginx
server {
    listen 80;
    server_name ruxum.dev www.ruxum.dev;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

---

## Option 3: Astro SSR (If you need dynamic content later)

If you eventually need server-side rendering, use Astro's Node adapter:

```bash
# Install Node adapter
npm install @astrojs/node
```

Update `astro.config.mjs`:
```javascript
import node from '@astrojs/node';

export default defineConfig({
  output: 'server',  // Change from 'static'
  adapter: node({
    mode: 'standalone'
  }),
  // ... rest of config
});
```

Then build and run:
```bash
npm run build
NODE_ENV=production node ./dist/server/entry.mjs
```

---

## Deployment Workflow (Update Your Site)

### Quick Update Script
Create `deploy.sh`:

```bash
#!/bin/bash
set -e

cd /home/ec2-user/ruxum

echo "📥 Pulling latest code..."
git pull origin main

echo "📦 Installing dependencies..."
npm install
cd www && npm install && cd ..

echo "🏗️  Building static site..."
npm --prefix www run build

echo "♻️  Restarting Nginx..."
sudo systemctl restart nginx

echo "✅ Deployment complete!"
```

Make it executable:
```bash
chmod +x deploy.sh
```

Run it:
```bash
./deploy.sh
```

---

## Monitoring & Maintenance

### Check logs
```bash
# Nginx
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log

# PM2 (if using Node.js server)
pm2 logs
```

### Monitor disk space
```bash
df -h
```

### Backup
```bash
tar -czf ruxum-backup-$(date +%Y%m%d).tar.gz /home/ec2-user/ruxum/www/dist
```

### Security updates
```bash
sudo yum update -y
sudo yum upgrade -y
```

---

## Troubleshooting

### "Port 80 already in use"
```bash
sudo lsof -i :80
# Kill the process if needed
sudo kill -9 <PID>
```

### "Permission denied" when accessing files
```bash
# Ensure Nginx can read files
sudo chown -R nginx:nginx /home/ec2-user/ruxum/www/dist
sudo chmod -R 755 /home/ec2-user/ruxum/www/dist
```

### Nginx not starting
```bash
sudo nginx -t  # Check syntax
journalctl -u nginx -n 50 --no-pager  # Check logs
```

### 404 on refresh (for SPA routing)
Make sure your Nginx config has:
```nginx
location / {
    try_files $uri $uri/ /index.html;
}
```

---

## Security Checklist

- [ ] Use HTTPS (Let's Encrypt certificate)
- [ ] Configure security headers in Nginx
- [ ] Enable SSH key authentication (disable password login)
- [ ] Configure Security Groups to allow only port 80/443
- [ ] Regular backups of static files
- [ ] Monitor disk space and memory
- [ ] Keep system and packages updated

---

## File Locations Reference

```
/home/ec2-user/ruxum/              # Project root
├── www/
│   ├── dist/                      # Static files (served by Nginx)
│   ├── src/
│   └── package.json
├── src/
└── package.json

/etc/nginx/conf.d/ruxum.conf       # Nginx config
/var/log/nginx/                    # Nginx logs
```

---

## Expected Performance

- Page load: < 1 second
- CSS/JS delivery: < 200ms
- Static file serving: Nginx handles this efficiently
- Memory usage: ~50-100MB for static site
- CPU usage: Minimal (just Nginx)

---

## When to Use Each Option

**Use Nginx + Static (Option 1):**
- ✅ High performance
- ✅ Low resource usage
- ✅ Best for static content
- ✅ Industry standard
- Recommended for your use case

**Use Node.js + PM2 (Option 2):**
- When you want Node.js for future features
- If you need custom middleware
- For easier Node.js development

**Use Astro SSR (Option 3):**
- If you need server-side rendering
- If you plan to add dynamic features
- If you need server-side authentication

---

**Status**: Ready for EC2 deployment  
**Recommended**: Option 1 (Nginx + Static Files)  
**Next steps**: Choose your deployment option and follow the steps above
