# jj tag list

`jj` 저장소의 태그를 표시.

## Usage
- `jj tag l` : 모든 태그 목록 표시
- `jj tag l "패턴"` : 패턴과 일치하는 태그 목록 표시
- `jj tag l "하위문자열:릴리스"` : 부분 문자열 패턴과 일치하는 태그 목록 표시
- `jj tag l --sort committer-date-` : 커미터 날짜 기준(최신순)으로 정렬하여 태그 목록 표시
- `jj tag l --sort name-` : 이름 기준 내림차순으로 정렬하여 태그 목록 표시
- `jj tag l -T "템플릿"` : 사용자 지정 템플릿으로 태그 목록 표시

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag-list
