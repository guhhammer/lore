# gpg-connect-agent

Communicate with a running agent.

## Usage
- `gpg-connect-agent [options][commands]` : the general form, as the manual gives it
- `gpg-connect-agent --keyboxd` : connect to a running keybox daemon instead of to the gpg-agent
- `gpg-connect-agent --raw-socket name` : connect to socket name assuming this is an Assuan style server
- `gpg-connect-agent --verbose` : output additional information while running
- `gpg-connect-agent --quiet` : try to be as quiet as possible

## Categories
security
