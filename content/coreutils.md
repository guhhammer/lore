# coreutils

Cross-platform reimplementation of the GNU coreutils in Rust Language. Includes a multi-call binary from which the utils can be invoked. This reduces the binary size and can be useful for portability.

## Usage
- `coreutils util util_options` : run a utility with arguments
- `coreutils ls -l` : list files in long format
- `coreutils ls --help` : display help for `ls`

## See also
- toybox
- busybox

## Links
- More information: https://uutils.github.io/coreutils/docs/multicall.html
- tldr page: https://tldr.inbrowser.app/pages/common/coreutils
