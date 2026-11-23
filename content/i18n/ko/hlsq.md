# hlsq

HLS 매니페스트를 색상과 기본 필터링 기능으로 표시하는 도구.

## Usage
- `curl --silent url | hlsq` : URL에서 HLS 매니페스트 보기
- `cat 경로/대상/파일.m3u8 | hlsq` : 파일에서 HLS 매니페스트 보기
- `hlsq -watch -url 주소` : URL을 지속적으로 다시 가져와 출력 갱신
- `cat 경로/대상/파일.m3u8 | hlsq -query 'type = SUBTITLES'` : 속성의 문자열 값을 기준으로 멀티 변형 플레이리스트 필터링
- `cat 경로/대상/파일.m3u8 | hlsq -query 'bandwidth > 1000000'` : 속성의 숫자 값을 기준으로 멀티 변형 플레이리스트 필터링

## Links
- More information: https://github.com/soldiermoth/hlsq/
