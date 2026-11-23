# blackfire

Monitor, profile, and test a PHP application.

## Usage
- `blackfire config` : initialize and configure the Blackfire client
- `blackfire agent` : launch the Blackfire agent
- `blackfire agent --socket="tcp://127.0.0.1:8307"` : launch the Blackfire agent on a specific socket
- `blackfire run php path/to/file.php` : run the profiler on a specific program
- `blackfire --samples 10 run php path/to/file.php` : run the profiler and collect 10 samples
- `blackfire --json run php path/to/file.php` : run the profiler and output results as JSON
- `blackfire upload path/to/file` : upload a profiler file to the Blackfire web service
- `blackfire status` : view the status of profiles on the Blackfire web service

## Categories
files, network, development

## Links
- More information: https://www.blackfire.io/
- tldr page: https://tldr.inbrowser.app/pages/common/blackfire
