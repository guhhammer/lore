# npm pkg

`package.json`의 속성을 조회하거나 수정하는 명령어.

## Usage
- `npm pkg get 이름` : 특정 속성 값을 조회
- `npm pkg get 이름|버전|...` : 여러 속성 값 한 번에 조회
- `npm pkg get 이름 버전 --ws` : 모든 워크스페이스에서 여러 값 조회
- `npm pkg get contributors[0].email` : 중첩 또는 배열 속성 값 조회
- `npm pkg set 속성=값` : 속성을 특정 값으로 설정
- `npm pkg set 속성1=값1 속성2=값2` : 여러 속성 한 번에 설정
- `npm pkg delete scripts.build` : `package.json`에서 특정 속성 삭제
- `npm pkg fix` : `package.json`의 일반적인 오류 자동 수정

## Links
- More information: https://docs.npmjs.com/cli/npm-pkg/
