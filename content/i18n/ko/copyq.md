# copyq

고급 기능을 갖춘 클립보드 매니저.

## Usage
- `copyq` : copyQ를 시작하여 클립보드 기록 저장
- `copyq clipboard` : 현재 클립보드 내용 표시
- `copyq add -- 텍스트1 텍스트2 텍스트3` : 클립보드 기록에 원본 텍스트 삽입
- `copyq add 첫째 줄\n둘째 줄` : 이스케이프 시퀀스 ('\n', '\t')가 포함 된 텍스트를 클립보드 기록에 삽입
- `copyq read 0 1 2` : 클립보드 기록에서 처음 3개 항목의 내용을 출력
- `copyq <파일.txt copy` : 파일 내용을 클립보드에 복사
- `copyq <이미지.jpg copy image/jpeg` : JPEG 이미지를 클립보드에 복사

## Links
- More information: https://copyq.readthedocs.io/en/latest/command-line.html
