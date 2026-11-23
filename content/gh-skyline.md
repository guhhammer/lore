# gh skyline

Generate a 3D model of your GitHub contribution history. By default, it will create a `{username}-{year}-github-skyline.stl` file in the current directory.

## Usage
- `gh skyline` : generate a skyline STL file for the current year and authenticated user
- `gh skyline -u username -y year` : generate a skyline for a specific user and year
- `gh skyline -u username -y first_year-last_year` : generate a skyline for a range of years
- `gh skyline -u username -f` : generate a full skyline (from the user's join year to the current year)
- `gh skyline -d` : enable debug logging
- `gh skyline -o path/to/output_file.stl` : generate a skyline and specify the output file path
- `gh skyline -u username -w` : open the GitHub profile for a specific user
- `gh skyline -h` : display help

## Categories
files, users, shell

## Links
- More information: https://github.com/github/gh-skyline
- tldr page: https://tldr.inbrowser.app/pages/common/gh-skyline
