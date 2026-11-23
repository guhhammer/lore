# gitlab-runner

Manage GitLab runners.

## Usage
- `sudo gitlab-runner register --url https://gitlab.example.com --registration-token token --name name` : register a runner
- `sudo gitlab-runner register --url https://gitlab.example.com --registration-token token --name name --executor docker` : register a runner with a Docker executor
- `sudo gitlab-runner unregister --name name` : unregister a runner
- `sudo gitlab-runner status` : display the status of the runner service
- `sudo gitlab-runner restart` : restart the runner service
- `sudo gitlab-runner verify` : check if the registered runners can connect to GitLab

## Categories
network, users

## Links
- More information: https://docs.gitlab.com/runner/
- tldr page: https://tldr.inbrowser.app/pages/common/gitlab-runner
