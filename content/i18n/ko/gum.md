# gum

매력적인 쉘 스크립트 만들기.

## Usage
- `gum choose "옵션_1" "옵션_2" ...` : `stdout`으로 출력할 특정 옵션을 대화형으로 선택
- `gum input --placeholder "값"` : 사용자가 특정 자리 표시자와 함께 문자열을 입력할 수 있는 대화형 프롬프트를 열기
- `gum confirm "Continue?" --default=false --affirmative "Yes" --negative "No" && echo "Yes selected" || echo "No selected"` : 대화형 확인 프롬프트를 열고 `<0>` 또는 `<1>`로 종료
- `gum spin -s dot|line|minidot|jump|pulse|points|globe|moon|monkey|meter|hamburger --title "loading..." 명령어` : 명령이 실행되는 동안 텍스트와 함께 스피너를 표시
- `gum format -t emoji ":smile: :heart: hello"` : 이모티콘을 포함하도록 텍스트 형식을 지정
- `gum write >경로/대상/파일` : 여러 줄의 텍스트를 대화식으로 프롬프트하고 `data.txt`에 작성

## Links
- More information: https://github.com/charmbracelet/gum#tutorial
