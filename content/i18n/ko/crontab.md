# crontab

현재 사용자에 대해 일정한 주기로 실행할 cron 작업을 설정.

## Usage
- `crontab -e` : 현재 사용자의 crontab 파일 편집
- `sudo crontab -e -u 사용자` : 특정 사용자의 crontab 파일 편집
- `crontab 경로/대상/파일` : 지정한 파일의 내용으로 현재 crontab을 교체
- `crontab -l` : 현재 사용자의 cron 작업 목록 보기
- `crontab -r` : 현재 사용자의 모든 cron 작업 제거
- `0 10 * * * 실행_할_명령` : 매일 10:00에 실행되는 작업 예 (*: 모든 값을 의미)
- `*/10 * * * * 실행_할_명령` : 10분마다 실행되는 cron 작업 예
- `30 2 * * Fri /경로/대상/script.sh` : 매주 금요일 02:30에 특정 스크립트를 실행하는 cron 작업 예

## Links
- More information: https://manned.org/crontab
