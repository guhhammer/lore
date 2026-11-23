# systemd-nspawn

경량 컨테이너에서 명령이나 운영 체제를 실행.

## Usage
- `systemd-nspawn -D 경로/대상/컨테이너_루트` : 컨테이너에서 명령 실행
- `systemd-nspawn -b -D 경로/대상/컨테이너_루트` : 컨테이너에서 전체 Linux 기반 운영 체제 실행
- `systemd-nspawn -D 경로/대상/컨테이너_루트 -a` : 지정된 명령을 컨테이너에서 PID 2로 실행 (PID 1 대신)하고, 초기화 프로세스 스텁 사용
- `systemd-nspawn -M 컨테이너_이름 --hostname 컨테이너_호스트 -D 경로/대상/컨테이너_루트` : 머신 이름과 호스트 이름 지정

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-nspawn.html
