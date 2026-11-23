# apt-key

Debian 및 Ubuntu의 APT 패키지 관리자를 위한 키 관리 도구.

## Usage
- `apt-key list` : 신뢰할 수 있는 키 나열
- `apt-key add 공개_키_파일.asc` : 신뢰할 수 있는 키 저장소에 키 추가
- `apt-key del 키_id` : 신뢰할 수 있는 키 저장소에서 키 삭제
- `wget -qO - https://host.tld/filename.key | apt-key add -` : 원격 키를 신뢰할 수 있는 키 저장소에 추가
- `apt-key adv --keyserver pgp.mit.edu --recv key_id` : 키 ID만 사용하여 키서버에서 키 추가

## Links
- More information: https://manned.org/apt-key
