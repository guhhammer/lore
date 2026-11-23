# setfiles

Set SELinux file security contexts based on policy rules. Similar to `restorecon` but reads contexts from a file_contexts file.

## Usage
- `sudo setfiles /etc/selinux/targeted/contexts/files/file_contexts path/to/directory` : set file contexts according to the default policy file
- `sudo setfiles /etc/selinux/targeted/contexts/files/file_contexts path/to/directory -v` : set file contexts recursively and show changes
- `sudo setfiles /etc/selinux/targeted/contexts/files/file_contexts path/to/directory -n` : preview what would be changed without actually modifying contexts
- `sudo setfiles /etc/selinux/targeted/contexts/files/file_contexts path/to/directory -v -F` : set file contexts and verify them
- `sudo setfiles /etc/selinux/targeted/contexts/files/file_contexts path/to/new_directory -r path/to/old_directory` : use a specific root path for context matching

## See also
- restorecon
- semanage fcontext
- fixfiles

## Categories
files, users

## Links
- More information: https://manned.org/setfiles
- tldr page: https://tldr.inbrowser.app/pages/linux/setfiles
