# grype

Vulnerability scanner for container images and filesystems.

## Usage
- `grype image:tag` : scan a container image
- `grype image:tag -o json|table|cyclonedx|cyclonedx-json|sarif|template` : scan an image and display results in a specific format
- `grype image:tag --only-fixed` : scan an image, ignoring unfixed vulnerabilities
- `grype image:tag -f negligible|low|medium|high|critical` : scan an image and fail on vulnerabilities with a severity at or above the given level
- `grype path/to/directory --file path/to/report` : scan a local directory and save the report to a file
- `grype db update` : update the vulnerability database
- `grype db status` : display the current database status
- `grype -h` : display help

## Categories
containers, media, files

## Links
- More information: https://oss.anchore.com/docs/reference/grype/cli
- tldr page: https://tldr.inbrowser.app/pages/common/grype
