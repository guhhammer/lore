# zrok

Expose local services and files to the internet. Part of the OpenZiti project, offering secure, zero-trust sharing.

## Usage
- `zrok invite` : request an invitation to use the public zrok service (run this first)
- `zrok enable your_token` : enable the zrok environment with the token from an invitation email
- `zrok share public http://localhost:8080` : create a publicly accessible URL for a local web server
- `zrok share private http://localhost:3000` : create a secure share accessible only with a unique token
- `zrok access private share_token` : access a private share created by another user
- `zrok share public --backend-mode web path/to/directory` : serve the contents of a local directory as a simple website
- `zrok status` : display the status of the zrok environment and active shares

## Categories
network, security, system

## Links
- More information: https://docs.zrok.io/
- tldr page: https://tldr.inbrowser.app/pages/common/zrok
