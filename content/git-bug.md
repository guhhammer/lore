# git bug

A distributed bug tracker that uses Git's internal storage, so no files are added to your project. You may submit your problems to the same Git remote you use to interact with others, much like commits and branches.

## Usage
- `git bug user create` : create a new identity
- `git bug add` : create a new bug
- `git bug push` : push a new bug entry to a remote
- `git bug pull` : pull for updates
- `git bug ls` : list existing bugs
- `git bug ls "status:open sort:edit"` : filter and sort bugs using a query
- `git bug ls "search_query" baz` : search for bugs by text content

## Categories
version-control, development

## Links
- More information: https://github.com/git-bug/git-bug/blob/master/doc/md/git-bug.md
- tldr page: https://tldr.inbrowser.app/pages/common/git-bug
- Wikipedia: https://en.wikipedia.org/wiki/Git
