# veritysetup

Manage dm-verity (block level verification) volumes.

## Usage
- `veritysetup <action> [<options>] <action args>` : the general form, as the manual gives it
- `veritysetup --batch-mode` : do not ask for confirmation
- `veritysetup --data-blocks=blocks` : size of data device used in verification
- `veritysetup --data-block-size=bytes` : used block size for the data device
- `veritysetup --debug` : run in debug mode with full diagnostic logs

## Links
- Documentation: https://gitlab.com/cryptsetup/cryptsetup/wikis/DMVerity
