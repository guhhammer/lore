# udevadm

Linux `udev` 관리 도구.

## Usage
- `sudo udevadm monitor` : 모든 장치 이벤트 모니터링
- `sudo udevadm monitor -k` : 커널에 의해 전송된 `uevent` 출력
- `sudo udevadm monitor -u` : `udev`에 의해 처리된 후의 장치 이벤트 출력
- `sudo udevadm info -a /dev/sda` : 장치 `/dev/sda`의 속성 나열
- `sudo udevadm control -R` : 모든 `udev` 규칙 다시 로드
- `sudo udevadm trigger` : 모든 `udev` 규칙 실행 트리거
- `sudo udevadm test /dev/sda` : `/dev/sda` 로딩을 시뮬레이션하여 이벤트 실행 테스트

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/udevadm.html
