# aa-disable

AppArmor 보안 정책 비활성화.

## Usage
- `sudo aa-disable 경로/대상/프로필1 경로/대상/프로필2 ...` : 프로필 비활성화
- `sudo aa-disable --dir 경로/대상/프로필들` : 디렉토리 내의 프로필 비활성화 (기본값은 `/etc/apparmor.d`)

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-disable.8
