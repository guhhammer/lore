# kmscon

TTY에서 텍스트 모드 대신 프레임버퍼를 사용하여 터미널을 표시.

## Usage
- `sudo kmscon` : 사용 가능한 첫 번째 TTY에서 `kmscon` 시작
- `sudo kmscon --vt /dev/ttyX|ttyX|X` : 지정한 TTY에서 `kmscon` 시작
- `sudo kmscon --mouse` : 마우스 지원 활성화
- `sudo kmscon -l 명령어` : 로그인에 사용할 명령 지정
- `systemctl enable --now kmsconvt@ttyX` : 지정한 터미널에서 항상 `kmscon` 시작

## Links
- More information: https://manned.org/kmscon
