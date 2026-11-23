# msfconsole

Console for the Metasploit Framework. Note: Run `sudo msfdb init` to enable the Metasploit database backend prior to launching `msfconsole`.

## Usage
- `sudo msfconsole` : launch the interactive console (append `--quiet` to suppress the startup banner)
- `sudo msfconsole -x "use auxiliary/scanner/portscan/tcp; set PORTS 80,443; set RHOSTS example.com; run; quit"` : execute console commands (Note: Use `;` for passing multiple commands)
- `sudo msfconsole -r path/to/file.rc` : run a specific resource file
- `show auxiliary|encoders|evasion|exploits|nops|payloads|post` : [Interactive] Show specific type of modules
- `use auxiliary/scanner/portscan/syn` : [Interactive] Use a module
- `show options` : [Interactive] Show module options (module needs to be loaded first)
- `set variable_name value` : [Interactive] Set value of variable
- `run|exploit` : [Interactive] Run a module (module needs to be loaded and options need to be set first)

## Categories
users, development, system

## Links
- More information: https://docs.rapid7.com/metasploit/msf-overview/
- tldr page: https://tldr.inbrowser.app/pages/common/msfconsole
