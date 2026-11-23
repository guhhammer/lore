# groff

`troff` 및 `nroff` 조판 유틸리티를 GNU로 대체.

## Usage
- `groff 경로/대상/입력파일.roff >경로/대상/출력파일.ps` : postScript 프린터의 출력 형식을 지정하고, 출력을 파일에 저장
- `groff -man -T ascii 경로/대상//manpage.1 | less --RAW-CONTROL-CHARS` : ASCII 출력 장치를 사용하여 매뉴얼 페이지를 렌더링하고, 호출기를 사용하여 표시
- `groff -man -T html 경로/대상//manpage.1 >경로/대상//manpage.html` : 매뉴얼 페이지를 HTML 파일로 렌더링
- `groff -t -p -me -T pdf 경로/대상/입력파일.me >경로/대상/출력파일.pdf` : [me] 매크로 세트를 사용하여 테이블(tables) 및 그림(pictures)이 포함된 roff 파일을 PDF로 조판하고, 출력을 저장
- `eval "$(grog -T utf8 경로/대상/입력파일.me)"` : `grog` 유틸리티에서 추측한 전처리기 및 매크로 옵션을 사용하여 `groff` 명령을 실행

## Links
- More information: https://www.gnu.org/software/groff/manual/groff.html.node/Groff-Options.html
