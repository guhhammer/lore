# xidel

HTML/XML 페이지 및 JSON API에서 데이터를 다운로드하고 추출.

## Usage
- `xidel https://www.google.com/search?q=test -e "//a/extract(@href, 'url[?]q=([^&]+)&', 1)[. != '']"` : google 검색으로 찾은 모든 URL 출력
- `xidel https://www.google.com/search?q=test -f "//a/extract(@href, 'url[?]q=([^&]+)&', 1)[. != '']" -e //title --download '{$host}/'` : google 검색으로 찾은 모든 페이지의 제목을 출력하고 다운로드
- `xidel https://example.org -f //a -e //title` : 페이지의 모든 링크를 따라가서 제목을 XPath로 출력
- `xidel https://example.org -f "css('a')" --css title` : 페이지의 모든 링크를 따라가서 제목을 CSS 선택자로 출력
- `xidel https://example.org -f "<a>{.}</a>*" -e "<title>{.}</title>"` : 페이지의 모든 링크를 따라가서 제목을 패턴 매칭으로 출력
- `xidel 경로/대상/example.xml -e "<x><foo>ood</foo><bar>{.}</bar></x>"` : example.xml에서 패턴을 읽고 "ood"를 포함한 요소가 있는지 확인(없으면 실패)
- `xidel http://stackoverflow.com/feeds -e "<entry><title>{title:=.}</title><link>{uri:=@href}</link></entry>+"` : 패턴 매칭을 사용하여 제목과 URL을 포함한 최신 Stack Overflow 질문 출력
- `xidel https://reddit.com -f "form(css('form.login-form')[1], {'user': '$your_username', 'passwd': '$your_password'})" -e "css('#mail')/@title"` : 읽지 않은 Reddit 메일 확인, 웹 스크래핑, CSS, XPath, JSONiq 및 자동 양식 평가 조합

## Links
- More information: https://www.videlibri.de/xidel.html#examples
