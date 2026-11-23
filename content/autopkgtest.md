# autopkgtest

Run tests on Debian packages.

## Usage
- `autopkgtest -- null` : build the package in the current directory and run all tests directly on the system
- `autopkgtest --test-name=test_name -- null` : run a specific test for the package in the current directory
- `autopkgtest package -- null` : download and build a specific package with `apt-get`, then run all tests
- `autopkgtest -- chroot path/to/new_root` : test the package in the current directory using a new root directory
- `autopkgtest -B -- null` : test the package in the current directory without rebuilding it

## Categories
development, packages, files

## Links
- More information: https://manned.org/autopkgtest
- tldr page: https://tldr.inbrowser.app/pages/linux/autopkgtest
