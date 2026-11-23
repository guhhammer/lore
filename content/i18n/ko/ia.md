# ia

`archive.org`와 상호작용하기 위한 커멘드라인 도구.

## Usage
- `ia configure` : API 키로 `ia`를 구성 (일부 기능은 이 단계 없이 작동하지 않음)
- `ia upload 식별자 경로/대상/파일 --metadata="mediatype:data" --metadata="title:example"` : 하나 이상의 항목을 `archive.org`에 업로드
- `ia download 항목` : `archive.org`에서 하나 이상의 항목을 다운로드
- `ia delete 식별자 파일` : `archive.org`에서 하나 이상의 항목을 삭제
- `ia search 'subject:"subject" collection:collection'` : `archive.org`에서 검색하여, 결과를 JSON로 반환

## Links
- More information: https://archive.org/developers/internetarchive/cli.html
