# scrontab

Slurm 크론탭 파일을 관리합니다.

## Usage
- `scrontab 경로/대상/파일` : 지정된 파일에서 새 크론탭 설치
- `scrontab -e` : 현재 사용자의 크론탭 e편집
- `scrontab --user=사용자_id -e` : 지정된 사용자의 크론탭 e편집
- `scrontab -r` : 현재 크론탭 r제거
- `scrontab -l` : 현재 사용자의 크론탭을 `stdout`에 출력

## Links
- More information: https://slurm.schedmd.com/scrontab.html
