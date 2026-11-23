# in-toto-run

Generate link metadata while carrying out a supply chain step.

## Usage
- `in-toto-run -n tag -p . --signing-key key_file -- git tag v1.0` : tag a Git repo and signing the resulting link file
- `in-toto-run -n package -m project -p project.tar.gz -- tar czf project.tar.gz project` : create a tarball, storing files as materials and the tarball as product
- `in-toto-run -n review --signing-key key_file -m document.pdf -x` : generate signed attestations for review work
- `in-toto-run -n scan --signing-key key_file -p report.json -- /bin/sh -c "trivy --output report.json --format json path/to/image"` : scan the image using Trivy and generate link file

## Categories
files, development, text

## Links
- More information: https://in-toto.readthedocs.io/en/latest/command-line-tools/in-toto-run.html
- tldr page: https://tldr.inbrowser.app/pages/common/in-toto-run
