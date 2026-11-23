# pwsh

Shell and scripting language designed especially for system administration. This command refers to PowerShell version 6 and above (also known as PowerShell Core and cross-platform PowerShell). To use the original Windows version (5.1 and below, also known as the legacy Windows PowerShell), use `powershell` instead of `pwsh`.

## Usage
- `pwsh` : start an interactive shell session
- `pwsh -NoProfile` : start an interactive shell session without loading startup configs
- `pwsh -Command "echo 'powershell is executed'"` : execute specific commands
- `pwsh -File path/to/script.ps1` : execute a specific script
- `pwsh -Version version` : start a session with a specific version of PowerShell
- `pwsh -NoExit` : prevent a shell from exit after running startup commands
- `pwsh -InputFormat Text|XML` : describe the format of data sent to PowerShell
- `pwsh -OutputFormat Text|XML` : determine how an output from PowerShell is formatted

## Categories
shell, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/pwsh
