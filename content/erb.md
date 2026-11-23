# erb

Ruby Templating.

## Usage
- `erb [--version] [-UPdnvx] [-E ext[:int]] [-S level] [-T mode] [-r library] [--] [file ...]` : the general form, as the manual gives it
- `erb --version` : prints the version of erb
- `erb -P` : disables ruby code evaluation for lines beginning with %
- `erb -S level` : specifies the safe level in which eRuby script will run
- `erb -T mode` : specifies trim mode (default 0).  mode can be one of
