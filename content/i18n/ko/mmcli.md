# mmcli

ModemManager를 제어하고 모니터링.

## Usage
- `mmcli --list-modems` : 사용 가능한 모뎀 나열
- `mmcli --modem=모뎀` : 모뎀에 대한 정보 출력
- `mmcli --modem=모뎀 --enable` : 모뎀 활성화
- `sudo mmcli --modem=모뎀 --messaging-list-sms` : 모뎀에서 사용 가능한 SMS 메시지 나열
- `sudo mmcli --modem=모뎀 --messaging-delete-sms=경로/대상/메시지_파일` : 모뎀에서 메시지를 삭제, 경로 지정

## Links
- More information: https://www.freedesktop.org/software/ModemManager/man/latest/mmcli.1.html
