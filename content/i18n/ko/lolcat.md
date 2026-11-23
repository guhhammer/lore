# lolcat

콘솔에 출력(`cat`)되는 텍스트를 무지개 색상으로 표시.

## Usage
- `lolcat 경로/대상/파일` : 파일 내용을 무지개 색상으로 출력
- `fortune | lolcat` : 텍스트를 생성하는 명령의 결과를 무지개 색상으로 출력
- `lolcat -S 숫자 경로/대상/파일` : 시드를 지정하여 항상 동일한 색상 패턴 사용 (기본값: `0`이고, 무작위를 의미)
- `lolcat -F 숫자 경로/대상/파일` : 무지개 색상 변화 빈도 조절 (기본값: `0.1`)
- `lolcat -p 숫자 경로/대상/파일` : 무지개 색상 퍼짐 정도 조절 (기본값: `3`)
- `lolcat -a 경로/대상/파일` : 애니메이션 무지개 색상으로 출력
- `lolcat -t 경로/대상/파일` : 24비트(True Color) 무지개 색상으로 출력

## Links
- More information: https://manned.org/lolcat
