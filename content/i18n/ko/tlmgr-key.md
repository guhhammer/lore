# tlmgr key

TeX Live 데이터베이스를 검증하는 데 사용되는 GPG 키 관리.

## Usage
- `tlmgr key list` : teX Live의 모든 키 나열
- `sudo tlmgr key add 경로/대상/키.gpg` : 특정 파일에서 키 추가
- `cat 경로/대상/키.gpg | sudo tlmgr key add -` : `stdin`에서 키 추가
- `sudo tlmgr key remove 키_id` : ID로 특정 키 제거

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#key
