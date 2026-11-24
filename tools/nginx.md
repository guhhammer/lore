# nginx

nginx is a web server and reverse proxy. This page covers running it, checking its config and the usual server blocks for a site, a proxy and redirects.

## Tasks
### Run it
- `sudo nginx -t` : check the configuration for mistakes
- `sudo systemctl reload nginx` : apply a new config without dropping connections
- `sudo systemctl restart nginx` : full restart
- `sudo systemctl status nginx` : is it running
- `sudo nginx -T` : print the whole config nginx loaded
- `nginx -v` : the version
### Where things are
- `ls /etc/nginx/sites-available /etc/nginx/sites-enabled` : site files and the ones turned on (Debian, Ubuntu)
- `ls /etc/nginx/conf.d` : site files on Fedora and most others
- `sudo ln -s /etc/nginx/sites-available/NAME /etc/nginx/sites-enabled/` : turn a site on (Debian, Ubuntu)
- `tail -f /var/log/nginx/error.log` : follow the errors
- `tail -f /var/log/nginx/access.log` : follow the requests
### Config pieces (inside a server block)
- `listen 80;` : the port to listen on
- `server_name example.org www.example.org;` : the names this block answers
- `root /var/www/example;` : the folder of the site
- `index index.html;` : the default file
- `location / { try_files $uri $uri/ =404; }` : serve files, else a 404
- `location /api/ { proxy_pass http://127.0.0.1:3000; }` : pass a path to an app
- `return 301 https://$host$request_uri;` : redirect to HTTPS
### Proxy details
- `proxy_set_header Host $host;` : keep the original Host header
- `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;` : pass the visitor's address
- `proxy_set_header Upgrade $http_upgrade;` : let websockets through (with Connection "upgrade")
### HTTPS and limits
- `listen 443 ssl;` : listen for HTTPS
- `ssl_certificate /etc/letsencrypt/live/NAME/fullchain.pem;` : the certificate chain
- `ssl_certificate_key /etc/letsencrypt/live/NAME/privkey.pem;` : the private key
- `client_max_body_size 50m;` : allow bigger uploads than the 1m default
- `gzip on;` : compress text answers

## Notes
- Always run `nginx -t` before a reload: a broken config will stop the reload.
- A `502 Bad Gateway` means the app behind `proxy_pass` is down or on another port.
- Files you need to read must be readable by the nginx user (www-data or nginx).
- Config lines end with a semicolon; a missing one is the most common error.
- certbot can write the HTTPS parts for you.

## See also
- certbot
- systemd
- curl
- ufw

## Links
- Docs: https://nginx.org/en/docs/
- Beginner's guide: https://nginx.org/en/docs/beginners_guide.html

## Categories
web services
