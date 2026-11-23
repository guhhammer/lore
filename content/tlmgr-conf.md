# tlmgr conf

Manage the TeX Live configuration.

## Usage
- `tlmgr conf` : show the current TeX Live configuration
- `tlmgr conf texmf|tlmgr|updmap` : show the current `texmf`, `tlmgr`, or `updmap` configuration
- `tlmgr conf texmf|tlmgr|updmap configuration_key` : show only a specific configuration option
- `tlmgr conf texmf|tlmgr|updmap configuration_key value` : set a specific configuration option
- `tlmgr conf texmf|tlmgr|updmap --delete configuration_key` : delete a specific configuration option
- `tlmgr conf texmf shell_escape 0` : disable the execution of system calls via `\write18`
- `tlmgr conf auxtrees show` : show all additional `texmf` trees

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#conf
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-conf
