# gh repo create

새로운 GitHub 저장소 생성.

## Usage
- `gh repo new` : 새로운 저장소를 대화형으로 생성
- `gh repo new -s . --private` : 현재 디렉터리로부터 비공개 저장소 생성
- `gh repo new -s . --public` : 현재 디렉터리로부터 공개 저장소 생성
- `gh repo new 저장소_이름 -d "저장소_설명" --public` : 이름과 설명을 지정하여 공개 저장소 생성
- `gh repo new 저장소_이름 -c --public|--private|--internal` : 저장소 생성 후 로컬로 clone 수행

## Links
- More information: https://cli.github.com/manual/gh_repo_create
