# aa-decode

AppArmor 감사 로그를 사람이 읽기 쉬운 형태로 디코딩하는 도구.

## Usage
- `aa-decode 16진수_문자열` : 16진수 문자열 디코딩
- `sudo aa-decode 로그파일` : 로그 파일 디코딩
- `sudo aa-decode - <로그파일` : `stdin`으로 전달된 로그 디코딩 (예: 파일 리다이렉션)
- `aa-decode -h` : 도움말 표시

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-decode.8
