# cupsd

CUPS 인쇄 서버용 서버 데몬.

## Usage
- `cupsd` : 백그라운드에서 `cupsd`를 데몬으로 시작
- `cupsd -f` : 포어그라운드에서 `cupsd`를 시작
- `cupsd -l` : 필요에 따라 `cupsd` 실행(launch) (일반적으로 `launchd` 또는 `systemd`에서 사용됨)
- `cupsd -c 경로/대상/cupsd.conf` : start `cupsd` using the specified [`c`]`upsd.conf` configuration file
- `cupsd -s 경로/대상/cups-파일.conf` : 지정된 `cups-file`[`s`]`.conf` 구성 파일을 사용하여 `cupsd`를 시작
- `cupsd -t -c path/to/cupsd.conf` : [`c`]`upsd.conf` 구성 파일에 오류가 있는지 확인(test)
- `cupsd -t -s 경로/대상/cups-파일.conf` : `cups-file`[`s`]`.conf` 구성 파일들에 오류가 있는지 확인(test)
- `cupsd -h` : 도움말 표시

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsd.html
