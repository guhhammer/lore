# ng test

프로젝트의 단위 테스트 실행. 참고: 일부 기능은 Vitest 등 추가 패키지가 필요할 수 있음.

## Usage
- `ng t` : 단위 테스트 실행
- `ng t -c 개발|프로덕션|...` : 특정 설정으로 테스트 실행
- `ng t --browsers firefox|webkit|chromium` : 테스트 실행에 사용할 브라우저 지정
- `ng t --coverage` : 코드 커버리지 생성
- `ng t --coverage --coverage-reporters cobertura|html|json|...` : 특정 형식의 커버리지 리포트 생성
- `ng t --debug` : 테스트 디버그 모드 실행
- `ng t --list-tests` : 실제 실행 없이 발견된 테스트 파일 목록 출력

## Links
- More information: https://angular.dev/cli/test
