# transmission-create

BitTorrent `.torrent` 파일 만들기.

## Usage
- `transmission-create -o 경로/대상/example.torrent -t 트래커_발표_url -s 2048 경로/대상/파일_또는_폴더` : 2048 KB를 조각 크기로 하여 토렌트 생성
- `transmission-create -p -o 경로/대상/example.torrent -t 트래커_발표_url -s 2048 경로/대상/파일_또는_폴더` : 2048 KB 조각 크기로 개인 토렌트 생성
- `transmission-create -o 경로/대상/example.torrent -t 트래커_url1 -c 코멘트 경로/대상/파일_또는_폴더` : 코멘트를 포함하여 토렌트 생성
- `transmission-create -o 경로/대상/example.torrent -t 트래커_url1 -t 트래커_url2 경로/대상/파일_또는_폴더` : 여러 트래커를 포함하여 토렌트 생성
- `transmission-create -h` : 도움말 표시

## Links
- More information: https://manned.org/transmission-create
