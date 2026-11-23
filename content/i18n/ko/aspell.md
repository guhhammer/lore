# aspell

대화형 맞춤법 검사기. 더 많은 정보: <http://aspell.net/man-html/index.html>.

## Usage
- `aspell check 경로/대상/파일` : 단일 파일의 철자 검사
- `cat 경로/대상/파일 | aspell list` : `stdin`에서 철자가 틀린 단어를 나열
- `aspell dicts` : 사용 가능한 사전적 언어 표시
- `aspell --lang cs` : 다른 언어로 `aspell`을 실행 (두 글자 ISO 639 언어 코드 사용)
- `cat 경로/대상/파일 | aspell --personal personal-word-list.pws list` : `stdin`에서 철자가 틀린 단어를 나열하고, 개인 단어 목록에서 단어를 무시
