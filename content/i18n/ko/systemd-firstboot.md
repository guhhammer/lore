# systemd-firstboot

시스템의 첫 부팅 시 또는 부팅 전에 기본 시스템 설정 초기화.

## Usage
- `sudo systemd-firstboot --root 경로/대상/루트_디렉토리` : 호스트 시스템의 루트 디렉토리 대신 지정된 디렉토리에서 작동
- `sudo systemd-firstboot --keymap 키맵` : 시스템 키보드 레이아웃 설정
- `sudo systemd-firstboot --hostname 호스트명` : 시스템 호스트명 설정
- `sudo systemd-firstboot --root-password 비밀번호` : 루트 사용자의 비밀번호 설정
- `sudo systemd-firstboot --prompt 설정` : 특정 기본 설정을 사용자에게 인터랙티브하게 요청
- `sudo systemd-firstboot --force` : 관련 파일이 이미 존재해도 강제로 설정 작성
- `sudo systemd-firstboot --reset` : `systemd-firstboot`에 의해 설정된 기존 파일 모두 제거
- `sudo systemd-firstboot --delete-root-password` : 시스템의 루트 사용자의 비밀번호 제거
