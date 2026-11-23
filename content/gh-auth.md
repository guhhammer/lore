# gh auth

Authenticate with a GitHub host.

## Usage
- `gh auth login` : log in with interactive prompt
- `echo your_token | gh auth login --with-token` : log in with a token from `stdin` (created in <https://github.com/settings/tokens>)
- `gh auth status` : check if you are logged in
- `gh auth logout` : log out
- `gh auth login -h github.example.com` : log in with a specific GitHub Enterprise Server
- `gh auth refresh` : refresh the session to ensure authentication credentials have the correct minimum scopes (removes additional scopes requested previously)
- `gh auth refresh -s repo,admin:repo_hook,admin:org,admin:public_key,admin:org_hook,...` : expand the permission scopes

## Categories
network, monitoring, users

## Links
- More information: https://cli.github.com/manual/gh_auth
- tldr page: https://tldr.inbrowser.app/pages/common/gh-auth
