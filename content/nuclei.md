# nuclei

Fast and customizable vulnerability scanner using a simple YAML-based DSL.

## Usage
- `nuclei -ut` : update `nuclei` templates to the latest released version (downloaded to `~/nuclei-templates` on macOS/Linux or `%USERPROFILE%\nuclei-templates` on Windows)
- `nuclei -tl -pt dns|file|http|headless|tcp|workflow|ssl|websocket|whois|code|javascript` : list all templates by specific protocol type
- `nuclei -as -u example.com` : run an automatic web scan using Wappalyzer technology detection for a specific target uRL/host
- `nuclei -s high,critical,... -pt http -u https://example.com -me path/to/directory` : run HTTP protocol type templates of specific severity, exporting results to markdown files inside a specific directory
- `nuclei -rl 150 -bs 25 -c 25 -silent -u https://example.com` : run all templates with a custom rate limit, maximum bulk size, and silent output (only findings shown)
- `nuclei -w workflows/wordpress-workflow.yaml -u https://example.com` : run a specific nuclei-bundled workflow against a target
- `nuclei -t path/to/nuclei-templates/http -u https://example.com -v -o path/to/results` : run one or more specific templates or directory with templates with verbose output in `stderr` and output detected issues/vulnerabilities to a file
- `nuclei -u https://example.com -ai "find admin login endpoints"` : use an AI prompt to dynamically generate a template to scan a target (projectdiscovery cloud pdcp API key needs to be configured using `nuclei -auth`)

## Categories
network, files, text

## Links
- More information: https://docs.projectdiscovery.io/opensource/nuclei/running
- tldr page: https://tldr.inbrowser.app/pages/common/nuclei
