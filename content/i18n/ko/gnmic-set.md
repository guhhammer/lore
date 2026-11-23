# gnmic set

gnmi 네트워크 장치 구성을 수정.

## Usage
- `gnmic -a 아이피:포트 set --update-path 경로 --update-value 값` : 경로의 값을 업데이트
- `gnmic -a 아이피:포트 set --update-path 경로 --update-file 경로/대상/파일` : JSON 파일의 내용과 일치하도록 경로 값을 업데이트
- `gnmic -a 아이피:포트 set --replace-path 경로 --replace-file 경로/대상/파일` : JSON 파일의 내용과 일치하도록 경로 값을 변경
- `gnmic -a 아이피:포트 set --delete 경로` : 주어진 경로에서 노드를 삭제

## Links
- More information: https://gnmic.openconfig.net/cmd/set/
