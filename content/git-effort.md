# git effort

Display how much activity a file has had, showing commits per file and "active days", i.e. the total number of days with contributions to the file. Part of `git-extras`.

## Usage
- `git effort` : display each file in the repository, showing commits and active days
- `git effort --above 5` : display files modified by a specific number of commits or more, showing commits and active days
- `git effort -- --author="username"` : display files modified by a specific author, showing commits and active days
- `git effort -- --since="last month"` : display files modified since a specific time/date, showing commits and active days
- `git effort path/to/file_or_directory1 path/to/file_or_directory2 ...` : display only the specified files or directories, showing commits and active days
- `git effort path/to/directory/*` : display all files in a specific directory, showing commits and active days

## Categories
files, version-control, desktop

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-effort
- tldr page: https://tldr.inbrowser.app/pages/common/git-effort
