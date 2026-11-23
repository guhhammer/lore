# gpg

GNU Privacy Guard.

## Usage
- `gpg --full-gen-key` : GPG 공개 및 개인 키를 대화형으로 생성
- `gpg --clearsign doc.txt` : 암호화 없이 `doc.txt`에 서명 (`doc.txt.asc`에 출력을 기록)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : alice@example.com 및 bob@example.com에 대해 `doc.txt`를 암호화하고 서명 (`doc.txt.gpg`로 출력)
- `gpg -c doc.txt` : 비밀번호 문구만으로 `doc.txt`를 암호화 (`doc.txt.gpg`로 출력)
- `gpg -d doc.txt.gpg` : `doc.txt.gpg` 복호화 (`stdout`으로 출력)
- `gpg --import public.gpg` : 공개 키 가져오기
- `gpg --export -a alice@example.com` : alice@example.com에 대한 공개 키 내보내기 (`stdout`으로 출력)
- `gpg --export-secret-keys -a alice@example.com` : alice@example.com의 개인 키 내보내기 (`stdout`으로 출력)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
