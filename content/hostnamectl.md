# hostnamectl

Get or set the hostname of the computer.

## Usage
- `hostnamectl` : get the hostname of the computer
- `sudo hostnamectl set-hostname "hostname"` : set the hostname of the computer
- `sudo hostnamectl set-hostname --static "hostname.example.com" && sudo hostnamectl set-hostname --pretty "hostname"` : set a pretty hostname for the computer
- `sudo hostnamectl set-hostname --pretty ""` : reset hostname to its default value

## Categories
users

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/hostnamectl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/hostnamectl
