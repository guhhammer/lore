# twine

Utility for publishing Python packages on PyPI.

## Usage
- `twine upload dist/*` : upload to PyPI
- `twine upload -r testpypi dist/*` : upload to the Test PyPI repository to verify things look right
- `twine upload -u username -p password dist/*` : upload to PyPI with a specified username and password
- `twine upload --repository-url repository_url dist/*` : upload to an alternative repository URL
- `twine check dist/*` : check that your distribution's long description should render correctly on PyPI
- `twine upload --config-file configuration_file dist/*` : upload using a specific pypirc configuration file
- `twine upload --skip-existing dist/*` : continue uploading files if one already exists (only valid when uploading to PyPI)
- `twine upload --verbose dist/*` : upload to PyPI showing detailed information

## Categories
network, packages

## Links
- More information: https://twine.readthedocs.io/en/stable/#commands
- tldr page: https://tldr.inbrowser.app/pages/common/twine
