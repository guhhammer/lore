# wscript

Run Windows Script Host programs in the graphical user interface (GUI).

## Usage
- `wscript //h:cscript|wscript` : set `cscript` (CLI) or `wscript` (GUI) as the default interpreter to run Windows Script Host programs
- `wscript path\to\script.vbs` : run a script
- `wscript //i|b path\to\script.vbs` : run script in the interactive or batch mode (enable or suppress alerts, prompts, and scripting errors)
- `wscript //logo|//nologo path\to\script.vbs` : show or hide the Windows Script Host logo before running (useful to get Host version / suppress outputs)
- `wscript //e:jscript|vbscript|custom_engine path\to\script.vbs` : specify a custom execution engine to run the script (useful for using custom script filenames)
- `wscript //t:seconds path\to\script.vbs` : set a timeout for running a given script (in seconds)
- `wscript //d path\to\script.vbs` : start the debugger and wait until a scripting error occurs
- `wscript //x path\to\script.vbs` : start debugging from the first script line in an IDE (e.g., Visual Studio)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## See also
- cscript

## Categories
development, network, files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/wscript
