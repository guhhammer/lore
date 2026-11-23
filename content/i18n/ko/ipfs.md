# ipfs

Inter Planetary File System. p2p기반 하이퍼미디어 프로토콜, 웹을 더 개방적으로 만드는 것을 목표로 함.

## Usage
- `ipfs add 경로/대상/파일` : 로컬 파일을 추가하고, pin 처리 후 해시 출력
- `ipfs add -r 경로/대상/디렉터리` : 로컬에서 파일시스템으로 디렉터리를 재귀적으로 추가하고 해시 출력
- `ipfs get hash -o 경로/대상/파일` : 원격 파일을 저장하고 이름 지정하지만, pin 하지 않음
- `ipfs pin add hash` : 원격 파일을 로컬에 pin
- `ipfs pin ls` : pin 설정된 파일 목록 출력
- `ipfs pin rm hash` : 로컬 저장소에서 pin 해제
- `ipfs repo gc` : 로컬 저장소에서 pin되지 않은 파일 정리

## Links
- More information: https://docs.ipfs.tech/reference/kubo/cli/
