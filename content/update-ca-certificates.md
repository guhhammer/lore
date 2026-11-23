# update-ca-certificates

Update the CA certificates bundle and regenerate `/etc/ssl/certs`.

## Usage
- `sudo update-ca-certificates` : update certificates
- `sudo update-ca-certificates -v` : update certificates in verbose mode
- `sudo update-ca-certificates -f` : perform a fresh update (remove all symlinks and regenerate)
- `sudo cp path/to/certificate.crt /usr/local/share/ca-certificates/ && sudo update-ca-certificates` : add a custom certificate (copy it first, then update)

## Categories
security, users

## Links
- More information: https://manned.org/update-ca-certificates
- tldr page: https://tldr.inbrowser.app/pages/linux/update-ca-certificates
