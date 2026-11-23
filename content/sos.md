# sos

A unified tool for collecting system logs and other debug information. Used by system administrators and support representatives to assist in troubleshooting.

## Usage
- `sudo sos report` : generate an archive of system information and configuration
- `sudo sos report --case-id case_id` : generate a report tagged with a support case ID
- `sudo sos report --upload` : generate a report and upload it to the distribution support target
- `sudo sos report -l` : list all available plugins and their options
- `sudo sos report --all-logs` : collect complete logs without size restriction
- `sudo sos report --batch` : generate a report non-interactively without prompts
- `sudo sos report --label label` : add a custom label to the generated archive name
- `sudo sos report --tmp-dir path/to/directory` : save the generated archive to a specific directory

## Categories
users, network, archives

## Links
- More information: https://github.com/sosreport/sos
- tldr page: https://tldr.inbrowser.app/pages/linux/sos
