# hadolint

Dockerfile linter.

## Usage
- `hadolint path/to/Dockerfile` : lint a Dockerfile
- `hadolint -f json path/to/Dockerfile` : lint a Dockerfile, displaying the output in JSON format
- `hadolint -f tty|json|checkstyle|codeclimate|codacy path/to/Dockerfile` : lint a Dockerfile, displaying the output in a specific format
- `hadolint --ignore DL3006 --ignore DL3008 path/to/Dockerfile` : lint a Dockerfile ignoring specific rules
- `hadolint --trusted-registry docker.io --trusted-registry example.com:5000 path/to/Dockerfile1 path/to/Dockerfile2 ...` : lint multiple Dockerfiles using specific trusted registries

## Categories
files, text

## Links
- More information: https://github.com/hadolint/hadolint#cli
- tldr page: https://tldr.inbrowser.app/pages/common/hadolint
