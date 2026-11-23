# pip hash

검증을 위한 패키지 아카이브 해시 계산.

## Usage
- `pip hash 경로/대상/패키지.whl` : 패키지 파일의 해시 생성
- `pip hash -a sha256|sha384|sha512|... 경로/대상/패키지.whl` : 특정 알고리즘을 사용하여 해시 생성
- `pip hash 경로/대상/패키지1.whl 경로/대상/패키지2.whl ...` : 여러 파일의 해시 생성
- `pip hash 경로/대상/패키지.tar.gz` : 다운로드된 아카이브 파일의 해시 생성

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_hash/
