# clamdscan

ClamAV 데몬을 사용하여 바이러스를 검사.

## Usage
- `clamdscan 경로/대상/파일_또는_디렉터리` : 취약점이 있는지 파일이나 디렉터리를 스캔
- `command | clamdscan -` : `stdin`로부터 데이터를 스캔
- `clamdscan --infected` : 현재 디렉터리를 검사하고 감염된 파일만 출력
- `clamdscan --log 경로/대상/로그_파일` : 스캔 보고서를 로그 파일로 인쇄
- `clamdscan --move 경로/대상/격리되는_디렉토리` : 감염된 파일을 특정 디렉토리로 이동
- `clamdscan --remove` : 감염된 파일을 제거
- `clamdscan --multiscan` : 여러 스레드를 사용하여 디렉터리를 검사
- `clamdscan --fdpass` : 파일을 데몬으로 스트리밍하는 대신 파일 설명자를 전달

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamdscan
