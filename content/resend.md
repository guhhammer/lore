# resend

Send and manage emails from the command-line.

## Usage
- `resend login` : log in to Resend (opens browser for authentication)
- `resend login --key re_xxxxxxxxx` : log in with an API key directly (for CI/agents)
- `resend emails send --from email@example.com --to recipient@example.com --subject "subject" --text "message"` : send an email
- `resend templates create --name "Welcome" --subject "subject" --html "<h1>Hello</h1>"` : create a template
- `resend emails send --to recipient@example.com --template template_id` : send an email using a template
- `resend domains ls` : list verified domains
- `resend whoami` : show current authentication status
- `resend doctor` : check CLI version, API key, and domain status

## Categories
text

## Links
- More information: https://github.com/resend/resend-cli#commands
- tldr page: https://tldr.inbrowser.app/pages/common/resend
