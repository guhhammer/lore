# svn

Subversion client tool.

## Usage
- `svn co file:///path/to/repository` : check out a working copy from a repository
- `svn up` : bring changes from the repository into the working copy
- `svn add path/to/file_or_directory` : put files and directories under version control, scheduling them for addition to repository. They will be added in next commit
- `svn ci -m commit_log_message path/to/directory` : send changes from your working copy to the repository
- `svn log -vl 10` : display changes from the last 10 revisions, showing modified files for each revision
- `svn h` : display help

## Categories
version-control, files, packages

## Links
- More information: https://svnbook.red-bean.com/en/1.7/svn-book.html#svn.ref.svn
- tldr page: https://tldr.inbrowser.app/pages/common/svn
- Wikipedia: https://en.wikipedia.org/wiki/Apache_Subversion
