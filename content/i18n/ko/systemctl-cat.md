# systemctl cat

systemd가 인식하는 유닛 파일의 전체 내용을 출력.

## Usage
- `systemctl cat 유닛` : 유닛 파일의 내용과 절대 경로 출력
- `systemctl cat 유닛1 유닛2 ...` : 여러 유닛 파일의 내용 출력
- `systemctl cat 템플릿@` : 템플릿 유닛 파일의 내용 출력
- `systemctl cat 유닛 --user` : 사용자 유닛 파일의 내용 출력
