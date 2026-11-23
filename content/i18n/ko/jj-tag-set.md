# jj tag set

`jj` 저장소의 태그를 생성하거나 업데이트.

## Usage
- `jj tag s 태그_이름` : 현재 작업 복사본 리비전을 가리키는 태그 생성
- `jj tag s 태그_이름 -r revision` : 지정한 리비전을 가리키는 태그 생성
- `jj tag s 태그1 태그2 ... -r revision` : 동일한 리비전을 가리키는 여러 태그 생성
- `jj tag s 태그_이름 -r revision --allow-move` : 기존 태그를 다른 리비전으로 이동
- `jj tag s 태그_이름 -r @-` : 현재 리비전의 부모를 가리키는 태그 생성

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag-set
