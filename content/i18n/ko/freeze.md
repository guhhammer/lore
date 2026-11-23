# freeze

코드 및 터미널 출력 이미지를 생성하는 도구. 지원 형식은 PNG, SVG, WebP입니다.

## Usage
- `freeze 경로/대상/파일` : 파일 기반으로 코드 이미지 생성
- `freeze 경로/대상/파일 -o 경로/대상/출력_이미지.png` : 출력 경로 지정
- `freeze -x 명령어` : 터미널 출력 이미지를 생성
- `freeze 경로/대상/파일 -i` : 출력 이미지를 대화형으로 사용자 지정
- `freeze 경로/대상/파일 -t dracula` : 문법 강조 테마 선택
- `freeze 경로/대상/파일 -c base|full|user` : 기본 설정 템플릿 사용
- `freeze 경로/대상/파일 --lines 시작,끝` : 특정 줄 범위만 캡처
- `freeze 경로/대상/파일 --show-line-numbers` : 줄 번호 표시

## Links
- More information: https://github.com/charmbracelet/freeze#flags
