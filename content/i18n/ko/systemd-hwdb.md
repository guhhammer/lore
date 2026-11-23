# systemd-hwdb

하드웨어 데이터베이스 관리 도구.

## Usage
- `sudo systemd-hwdb update` : `/etc/udev`의 바이너리 하드웨어 데이터베이스 업데이트
- `systemd-hwdb query 모달리아스` : 하드웨어 데이터베이스를 조회하고 특정 모달리아스에 대한 결과 출력
- `sudo systemd-hwdb -s update` : 구문 오류 발생 시 0이 아닌 종료 값을 반환하며 바이너리 하드웨어 데이터베이스 업데이트
- `sudo systemd-hwdb --usr update` : `/usr/lib/udev`의 바이너리 하드웨어 데이터베이스 업데이트
- `systemd-hwdb -r 경로/대상/루트 update` : 지정된 루트 경로의 바이너리 하드웨어 데이터베이스 업데이트

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-hwdb.html
