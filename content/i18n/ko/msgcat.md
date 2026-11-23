# msgcat

여러 개의 `.po` 번역 파일을 연결하거나 병합. 소프트웨어 현지화 과정에서 메시지 카탈로그를 결합하는 데 사용.

## Usage
- `msgcat 파일1.po 파일2.po ... -o 병합된_파일.po` : 여러 `.po` 파일을 하나로 병합
- `msgcat -f path/to/file_list.txt -o 병합된_파일.po` : 텍스트 파일에 나열된 입력 파일들을 병합
- `msgcat -t UTF-8 입력_파일.po -o 출력_파일.po` : 출력 인코딩 지정 (예: UTF-8)
- `msgcat -u 파일1.po 파일2.po ... -o 고유_파일.po` : 한 파일에만 존재하는 고유 메시지만 출력
- `msgcat --use-first 파일1.po 파일2.po ... -o 출력_파일.po` : 중복 메시지의 경우, 첫 번째 번역을 사용
- `msgcat -h` : 도움말 표시

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#msgcat-Invocation
