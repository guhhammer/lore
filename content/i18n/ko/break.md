# break

`for`, `while`, `until`, `select` 루프에서 빠져나오기.

## Usage
- `while :; do break; done` : 하나의 루프에서 빠져나오기
- `while :; do while :; do break 2; done; done` : 중첩된 루프에서 빠져나오기

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-break
