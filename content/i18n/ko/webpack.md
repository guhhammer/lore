# webpack

웹 프로젝트의 자바스크립트 파일과 기타 리소스를 단일 출력 파일로 묶어줍니다.

## Usage
- `webpack app.js bundle.js` : 진입점이 되는 파일에서 단일 출력 파일 생성
- `webpack app.js bundle.js --module-bind 'css=css'` : 자바스크립트 파일에서도 CSS 파일을 로드 (이 경우 CSS 파일에 CSS 로더를 사용합니다)
- `webpack -c webpack.config.js --progress` : 구성 파일(예, 입력 스크립트 및 출력 파일 이름 포함)을 전달하고 컴파일 진행률을 표시
- `webpack -w app.js bundle.js` : 프로젝트 파일 변경 시 자동으로 다시 컴파일

## Links
- More information: https://webpack.js.org/api/cli/
