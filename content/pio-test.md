# pio test

Run local tests on a PlatformIO project.

## Usage
- `pio test` : run all tests in all environments of the current PlatformIO project
- `pio test -e environment1 -e environment2` : test only specific environments
- `pio test -f "pattern"` : run only tests whose name matches a specific `glob` pattern
- `pio test -i "pattern"` : ignore tests whose name matches a specific `glob` pattern
- `pio test --upload-port upload_port` : specify a port for firmware uploading
- `pio test -c path/to/platformio.ini` : specify a custom configuration file for running the tests

## Categories
development, network, text

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/cmd_test.html
- tldr page: https://tldr.inbrowser.app/pages/common/pio-test
