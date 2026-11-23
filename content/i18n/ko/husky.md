# husky

네이티브 Git 훅을 쉽게 만들었습니다.

## Usage
- `husky install` : 현재 폴더에 Husky를 설치
- `husky install 경로/대상/폴더` : husky를 특정 폴더에 설치
- `husky set .husky/pre-push "명령어 명령어_인자"` : 특정 명령을 Git의 `pre-push` 훅으로 설정
- `husky add .husky/pre-commit "명령어 명령어_인자"` : 현재 `pre-commit` 훅에 특정 명령을 추가
- `husky uninstall` : 현재 폴더에서 Husky 훅 제거
- `husky` : 도움말 표시

## Links
- More information: https://typicode.github.io/husky/
