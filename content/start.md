# start

Start a command in a new console. Can also be used to run Windows programs registered in the `App Paths` Windows Registry despite not registered in the `%PATH%` environment variable. In PowerShell, this command is an alias of `Start-Process`. This documentation is based on the Command Prompt (`cmd`) version of `start`.

## Usage
- `tldr start-process` : view the documentation of the equivalent PowerShell command
- `start "window_title" cmd|powershell|python|...` : start a new shell window with a custom title
- `start "" "path\to\directory"` : open a specific directory on Windows Explorer (equivalent to `explorer path\to\directory`)
- `start path\to\file` : start a specific Windows program file (command-line programs will open on a new console window)
- `start msedge --inprivate example.com` : start a registered Windows program name and pass its command arguments
- `start /min|/max path\to\file.exe` : start a Windows program in minimized or maximized window mode
- `start /d path\to\directory command_or_program` : start a command or Windows program with a specified working directory
- `start /wait command_or_program` : do not finish this command until the target command or program exits

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, shell, documents, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/start
