# sudo

단일 명령을 슈퍼유저 또는 다른 사용자로 실행.

## Usage
- `sudo less /var/log/syslog` : 명령을 슈퍼유저로 실행
- `sudo -e /etc/fstab` : 기본 편집기로 파일을 슈퍼유저 권한으로 편집
- `sudo -u 사용자 -g 그룹 id -a` : 다른 사용자 및/또는 그룹으로 명령 실행
- `sudo !!` : 마지막 명령을 `sudo`로 접두사 붙여서 반복 (Bash, Zsh 등에서만 가능)
- `sudo -i` : 슈퍼유저 권한으로 기본 셸 시작하고 로그인 관련 파일(`.profile`, `.bash_profile` 등) 실행
- `sudo -s` : 환경을 변경하지 않고 슈퍼유저 권한으로 기본 셸 시작
- `sudo -i -u 사용자` : 지정된 사용자로 기본 셸 시작, 사용자의 환경을 로드하고 로그인 관련 파일(`.profile`, `.bash_profile` 등) 읽기
- `sudo -ll` : 호출한 사용자에 대해 허용된 (및 금지된) 명령 목록 표시

## Links
- More information: https://www.sudo.ws/sudo.html
