# get-wuhistory

Get the history of installed updates from Windows Update. Part of external `PSWindowsUpdate` module. This command can only be run under PowerShell.

## Usage
- `Get-WUHistory` : get list of update history
- `Get-WUHistory -Last 10` : list the last 10 installed updates
- `Get-WUHistory -MaxDate date` : list all updates installed from a specific date to today
- `Get-WUHistory -MaxDate (Get-Date).AddDays(-1)` : list all updates installed in the past 24 hours
- `Get-WUHistory -SendReport -PSWUSettings @{SmtpServer="smtp_server"; Port=smtp_port From="sender_email" To="receiver_email"}` : send the results via email (SMTP)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
shell, development, network, windows

## Links
- More information: https://github.com/mgajda83/PSWindowsUpdate
- tldr page: https://tldr.inbrowser.app/pages/windows/get-wuhistory
