# run0

권한을 대화식으로 상승시킵니다. `sudo`와 유사하지만, SUID 바이너리가 아니며 인증은 polkit을 통해 이루어지고 명령어는 `systemd` 서비스에서 호출됩니다.

## Usage
- `run0 명령어` : 명령어를 루트 사용자로 실행
- `run0 -u 사용자명|uid -g 그룹_이름|gid 명령어` : 다른 사용자 및/또는 그룹으로 명령어 실행

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/run0.html
