# erb3.3

Ruby Templating.

## Usage
- `erb [--version] [-UPdnvx] [-E ext[:int]] [-S level] [-T mode] [-r library] [--] [file ...]` : the general form, as the manual gives it
- `erb3.3 --version` : prints the version of erb
- `erb3.3 -P` : disables ruby code evaluation for lines beginning with %
- `erb3.3 -S level` : specifies the safe level in which eRuby script will run
- `erb3.3 -T mode` : specifies trim mode (default 0).  mode can be one of
