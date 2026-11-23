# crane

컨테이너 이미지 관리 도구. `pull`, `push`, `copy` 등 일부 하위 명령어는 별도의 사용법 문서를 제공.

## Usage
- `crane auth login 레지스트리 -u 사용자 -p 비밀번호` : 레지스트리에 로그인
- `crane catalog 레지스트리 --full-ref` : 레지스트리의 저장소 목록을 표시
- `crane ls 저장소 -o` : 저장소의 태그 목록을 표시
- `crane pull 이미지 tarball` : 원격 이미지를 참조로 가져와 로컬에 저장
- `crane push path/to/directory_or_tarball 이미지` : 로컬 이미지 내용을 원격 레지스트리에 푸시
- `crane tag 이미지 tag` : 원격 이미지에 태그를 효율적으로 추가
- `crane cp 소스 목적지 -a` : digest 값을 유지하면서 원격 이미지를 효율적으로 복사
- `crane delete 이미지` : 레지스트리에서 이미지 참조를 삭제
