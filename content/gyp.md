# gyp

Cross-platform makefile generator for Chromium.

## Usage
- `gyp [options] [build_file...]` : the general form, as the manual gives it
- `gyp --ignore-environment` : do not read options from environment variables
- `gyp --check` : take extra time to verify the syntax of each build_file
- `gyp --parallel` : use multiprocessing for speed (experimental)
- `gyp --toplevel-dir=<dir>` : directory to use as the root of the source tree
