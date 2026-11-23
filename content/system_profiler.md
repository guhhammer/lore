# system_profiler

Report system hardware and software configuration.

## Usage
- `system_profiler -detailLevel level` : display a report with specific details level (mini [no personal information], basic, or full)
- `system_profiler -xml >MyReport.spx` : display a full system profiler report which can be opened by `System Profiler.app`
- `system_profiler SPHardwareDataType SPSoftwareDataType` : display a hardware overview (Model, CPU, Memory, Serial, etc.) and software data (System, Kernel, Name, Uptime, etc.)
- `system_profiler SPHardwareDataType|grep "Serial Number (system)" | awk '{ print $4 }'` : print the system serial number
- `system_profiler SPPowerDataType | awk '/Wattage/ {print $3}'` : display the negotiated max wattage of the connected charger and cable

## Categories
desktop, documents, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/system_profiler.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/system_profiler
