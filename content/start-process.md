# start-process

Start a command in a new console. Can also be used to run Windows programs registered in the `App Paths` Windows Registry despite not registered in the `$Env:PATH` environment variable.

## Usage
- `Start-Process path\to\directory` : open a specific directory on Windows Explorer (equivalent to `explorer path\to\directory`)
- `Start-Process path\to\file` : start a specific Windows program file (command-line programs will open on a new console window)
- `Start-Process msedge --inprivate example.com` : start a registered Windows program name and pass its command arguments
- `Start-Process -WindowStyle Minimized|Maximized program` : start a Windows program in minimized or maximized window mode
- `Start-Process -WorkingDirectory path\to\directory command_or_program` : start a command or Windows program with a specified working directory
- `Start-Process -Wait command_or_program` : do not finish this command until the target command or program exits

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, processes, shell, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/start-process
