# gitlab-ctl

Manage the GitLab omnibus.

## Usage
- `sudo gitlab-ctl status` : display the status of every service
- `sudo gitlab-ctl status nginx` : display the status of a specific service
- `sudo gitlab-ctl restart` : restart every service
- `sudo gitlab-ctl restart nginx` : restart a specific service
- `sudo gitlab-ctl tail` : display the logs of every service and keep reading until `<Ctrl c>` is pressed
- `sudo gitlab-ctl tail nginx` : display the logs of a specific service
- `sudo gitlab-ctl kill nginx` : send the `SIGKILL` signal to specific service
- `sudo gitlab-ctl reconfigure` : reconfigure the application

## Categories
users, system, desktop

## Links
- More information: https://docs.gitlab.com/omnibus/maintenance/
- tldr page: https://tldr.inbrowser.app/pages/common/gitlab-ctl
