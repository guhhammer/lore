# sfdk qmltypes

Generate qmltypes files.

## Usage
- `sfdk qmltypes` : generate qmltypes files
- `sfdk qmltypes --no-deploy` : generate qmltypes files without deploying them (usually to emulator)
- `sfdk qmltypes --no-restore-emulator` : generate qmltypes files without restoring the emulator after deployment
- `sfdk qmltypes --restore-emulator` : generate qmltypes files and restore the emulator after deployment even on failure
- `sfdk qmltypes --no-keep` : generate qmltypes files without reverting changes which only include removal of statements with `sdk-make-qmltypes:keep` in comments

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sfdk-qmltypes
