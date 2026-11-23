# ssh-copy-id

원격 시스템의 authorized_keys에 공개 키 설치.

## Usage
- `ssh-copy-id 사용자_명@원격_호스트` : 키를 원격 시스템에 복사
- `ssh-copy-id -i 경로/대상/인증서 사용자_명@원격_호스트` : 지정된 공개 키를 원격 시스템에 복사
- `ssh-copy-id -i 경로/대상/인증서 -p 포트 사용자_명@원격_호스트` : 특정 포트로 지정된 공개 키를 원격 시스템에 복사

## Links
- More information: https://manned.org/ssh-copy-id
