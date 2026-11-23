# iexpress

Create self-extracting archive executable (`.exe`) or cabinet (`.cab`) files.

## Usage
- `iexpress /n path\to\file.sed` : build a self-extracting ZIP archive from a given SED file
- `iexpress /n /m path\to\file.sed` : run the build process in a minimized window
- `iexpress /n /q:a|u path\to\file.sed` : run the build process in a quiet mode, assuming the user account invoking this command as an administrator or regular user
- `iexpress /r:a|n|s path\to\file.sed` : ensure the `.exe` file to ask, never, or always restart the computer after installation

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, archives, development, windows

## Links
- More information: https://ss64.com/nt/iexpress.html
- tldr page: https://tldr.inbrowser.app/pages/windows/iexpress
