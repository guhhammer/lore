# runas

Run a program as another user.

## Usage
- `runas /user:.\Administrator "command"` : run a program as the Administrator local user
- `runas /user:domain\username "command"` : run a program as a specific user
- `runas /noprofile /user:domain\username "command"` : run a program without loading the user's profile
- `runas /user:domain\username cmd` : open Command Prompt as another user
- `runas /env /user:domain\username "notepad \"C:\path\to\file.txt\""` : run Notepad as a specific user with the current user's environment variables, opening a file with escaped quotes
- `runas /env /user:domain\username "mmc %windir%\system32\dsa.msc"` : run Active Directory Users and Computers as a specific user

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
users, files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/runas
- Wikipedia: https://en.wikipedia.org/wiki/Runas
