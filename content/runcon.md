# runcon

Run a program in a different SELinux security context.

## Usage
- `runcon` : print the security context of the current execution context
- `runcon -t domain_t command` : specify the domain to run a command in
- `runcon -r role_r command` : specify the context role to run a command with
- `runcon user_u:role_r:domain_t command` : specify the full context to run a command with

## See also
- secon

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/runcon
