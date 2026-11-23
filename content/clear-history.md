# clear-history

Delete entries from the PowerShell session command history. Note: `clhy` can be used as an alias for `Clear-History`.

## Usage
- `Clear-History` : clear all command history from current session
- `Clear-History -CommandLine "command"` : clear command by specific name
- `Clear-History -CommandLine "command1", "command2", ...` : clear multiple commands by name
- `Clear-History -Id id_number` : clear a specific history entry by ID
- `Clear-History -Id id1, id2, ...` : clear multiple IDs
- `Clear-History -Id (start_id..end_id)` : clear commands within a range of IDs
- `Clear-History -WhatIf` : simulate clearing the history, showing what would be deleted
- `Clear-History -Confirm` : ask for confirmation before clearing

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
shell, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/clear-history
