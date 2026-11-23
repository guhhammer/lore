# satis

The utility for the Satis static Composer repository.

## Usage
- `satis init satis.json` : initialize a Satis configuration
- `satis add repository_url` : add a VCS repository to the Satis configuration
- `satis build satis.json path/to/output_directory` : build the static output from the configuration
- `satis build --repository-url repository_url satis.json path/to/output_directory` : build the static output by updating only the specified repository
- `satis purge satis.json path/to/output_directory` : remove useless archive files

## Categories
packages, development, text

## Links
- More information: https://github.com/composer/satis
- tldr page: https://tldr.inbrowser.app/pages/common/satis
