# cscript

Run Windows Script Host programs in the command-line interface (CLI).

## Usage
- `cscript //h:cscript|wscript` : set `cscript` (CLI) or `wscript` (GUI) as the default interpreter to run Windows Script Host programs
- `cscript path\to\script.vbs` : run a script
- `cscript //i|b path\to\script.vbs` : run script in the interactive or batch mode (enable or suppress alerts, prompts, and scripting errors)
- `cscript //logo|//nologo path\to\script.vbs` : show or hide the Windows Script Host logo before running (useful to get Host version / suppress outputs)
- `cscript //e:jscript|vbscript|custom_engine path\to\script.vbs` : specify a custom execution engine to run the script (useful for using custom script filenames)
- `cscript //t:seconds path\to\script.vbs` : set a timeout for running a given script (in seconds)
- `cscript //d path\to\script.vbs` : start debugger and wait until a scripting error occurs
- `cscript //x path\to\script.vbs` : start debugging from the first script line in an IDE (e.g., Visual Studio)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## See also
- wscript

## Categories
development, network, files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/cscript
