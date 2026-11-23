# jj revert

지정한 리비전의 변경 사항을 되돌리는 역변경을 적용.

## Usage
- `jj revert -r revsets` : 지정한 revset의 변경 사항을 되돌리는 역변경 적용 (예: `B::D`, `A..D`, `B|C|D` 등)
- `jj revert -r revsets -d revsets` : 지정한 리비전 위에 역변경 적용
- `jj revert -r revsets -B revsets -A revsets` : 지정한 리비전의 앞이나 뒤에 역변경 적용

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-revert
