# svn changelist

Associate a changelist with a set of files.

## Usage
- `svn cl changelist_name path/to/file1 path/to/file2 ...` : add files to a changelist, creating the changelist if it does not exist
- `svn cl --remove path/to/file1 path/to/file2 ...` : remove files from a changelist
- `svn cl --remove -R --changelist changelist_name .` : remove the whole changelist at once
- `svn cl -R changelist_name path/to/directory1 path/to/directory2 ...` : add the contents of a space-separated list of directories to a changelist
- `svn commit --changelist changelist_name` : commit a changelist

## Categories
version-control, files

## Links
- More information: https://svnbook.red-bean.com/en/1.7/svn-book.html#svn.ref.svn.c.changelist
- tldr page: https://tldr.inbrowser.app/pages/common/svn-changelist
- Wikipedia: https://en.wikipedia.org/wiki/Apache_Subversion
