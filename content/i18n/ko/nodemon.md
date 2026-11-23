# nodemon

파일을 감시하고 변경 사항이 감지되면 Node 애플리케이션을 자동으로 재시작.

## Usage
- `nodemon 경로/대상/파일.js` : 지정된 파일을 실행하고 특정 파일의 변경 사항 감시
- `rs` : [대화형] 수동으로 nodemon 재시작
- `nodemon -i 경로/대상/파일_또는_폴더` : 특정 파일 무시
- `nodemon 경로/대상/파일.js 인수들` : node 애플리케이션에 인수 전달
- `nodemon 인수들 경로/대상/파일.js` : nodemon 인수가 아닌 경우 Node 자체에 인수 전달(예: `--inspect`)
- `nodemon -x "스크립트를_실행할_명령 옵션들" 경로/대상/스크립트` : 임의의 비-Node 스크립트 실행
- `nodemon -x "python 옵션들" 경로/대상/파일.py` : python 스크립트 실행

## Links
- More information: https://github.com/remy/nodemon/tree/main/doc/cli
