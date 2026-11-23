# puppet agent

Retrieve the client configuration from a Puppet server and apply it to the local host.

## Usage
- `puppet agent --test --server puppetserver_fqdn --serverport port --waitforcert poll_time` : register a node at a Puppet server and apply the received catalog
- `puppet agent` : run the agent in the background (uses settings from `puppet.conf`)
- `puppet agent --test` : run the agent once in the foreground, then exit
- `puppet agent --test --noop` : simulate running the agent without applying any changes
- `puppet agent --test --evaltrace` : log every resource being evaluated (even if nothing is being changed)
- `puppet agent --disable "message"` : disable the agent
- `puppet agent --enable` : enable the agent

## Categories
network, development

## Links
- More information: https://github.com/puppetlabs/puppet/blob/main/references/man/agent.md
- tldr page: https://tldr.inbrowser.app/pages/common/puppet-agent
