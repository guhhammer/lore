# ufw show

Display information about the running firewall.

## Usage
- `sudo ufw show added` : show added rules in the same format as they were entered
- `sudo ufw show listening` : show listening ports and the rules that apply to them
- `sudo ufw show raw` : show all `iptables` chains and rules of the running firewall
- `sudo ufw show builtins|before-rules|user-rules|after-rules|logging-rules` : show a specific part of the running ruleset

## Categories
users, security, desktop

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-show
