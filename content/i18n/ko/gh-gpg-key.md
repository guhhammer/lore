# gh gpg-key

인증된 GitHub 계정에 등록된 GPG 키 관리.

## Usage
- `gh gpg-key ls` : 인증된 GitHub 계정의 GPG 키 목록 표시
- `gh gpg-key add 경로/대상/키_파일` : 키 파일을 지정하여 GitHub 계정에 GPG 키 추가
- `gpg -a --export 키_아이디 | gh gpg-key add -` : 키 ID를 지정하여 인증된 GitHub 계정에 GPG 키 추가
- `gh gpg-key delete 키_아이디` : 인증된 GitHub 계정에서 GPG 키 삭제

## Links
- More information: https://cli.github.com/manual/gh_gpg-key
