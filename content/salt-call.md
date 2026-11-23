# salt-call

Invoke salt locally on a salt minion.

## Usage
- `salt-call state.highstate` : perform a highstate on this minion
- `salt-call state.highstate test=true` : simulate a highstate, computing all changes without actually performing them
- `salt-call -l debug state.highstate` : perform a highstate with verbose debugging output
- `salt-call grains.items` : list this minion's grains

## Links
- More information: https://docs.saltproject.io/en/latest/ref/cli/salt-call.html
- tldr page: https://tldr.inbrowser.app/pages/common/salt-call
