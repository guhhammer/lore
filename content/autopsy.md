# autopsy

Digital forensics platform for analyzing disk images and investigating filesystems.

## Usage
- `autopsy64.exe` : launch the Autopsy graphical interface
- `autopsy64.exe --createCase --caseName case_name --caseBaseDir path\to\base_directory` : create a new case and specify the base directory to store case files
- `autopsy64.exe --caseDir path\to\case_directory --addDataSource --dataSourcePath path\to\source` : add a data source to an existing case
- `autopsy64.exe --caseDir path\to\case_directory --generateReports` : generate a report for an existing case
- `autopsy64.exe --caseDir path\to\case_directory --listAllDataSources` : list all data sources in an existing case

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, containers, disks, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/autopsy
- Wikipedia: https://en.wikipedia.org/wiki/Autopsy_(software)
