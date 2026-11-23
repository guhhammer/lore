# get-wuapiversion

Get the Windows Update Agent version. Part of external `PSWindowsUpdate` module. This command can only be run under PowerShell.

## Usage
- `Get-WUApiVersion` : get the currently-installed Windows Update Agent version
- `Get-WUApiVersion -SendReport -PSWUSettings @{SmtpServer="smtp_server"; Port=smtp_port From="sender_email" To="receiver_email"}` : send the current configuration data via email (SMTP)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, network, system, windows

## Links
- More information: https://github.com/mgajda83/PSWindowsUpdate
- tldr page: https://tldr.inbrowser.app/pages/windows/get-wuapiversion
