# boxxy

XDG 표준을 따르지 않는 프로그램의 파일 위치를 재지정.

## Usage
- `boxxy 프로그램` : `~/.config/boxxy/boxxy.yaml`에 정의된 파일 리디렉션 규칙을 사용해 프로그램을 실행
- `boxxy scan` : 홈 디렉터리를 검사해 규칙 제안을 찾음
- `boxxy -t 프로그램` : 프로그램이 접근하는 파일을 추적하고, 보고서를 현재 디렉터리의 `boxxy-report.txt`에 저장
- `boxxy -r 경로/대상/파일_또는_디렉토리:경로/대상/리디렉션:파일|디렉토리 프로그램` : 터미널에서 직접 리디렉션 규칙을 전달
- `boxxy config` : 설정파일 보기
- `boxxy -h` : 도움말 표시

## Links
- More information: https://github.com/queer/boxxy
