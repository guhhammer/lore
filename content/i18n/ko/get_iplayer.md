# get_iplayer

BBC iPlayer 및 BBC Sounds용 인덱싱 도구 및 개인용 비디오 레코더.

## Usage
- `get_iplayer "프로그램_이름"` : 프로그램 이름으로 검색
- `get_iplayer "프로그램_이름" -g` : 검색 결과를 기반으로 프로그램 녹화
- `get_iplayer "https://www.bbc.co.uk/iplayer/episode/program_pid/name-of-show-episode-number-episode-title"` : BBC iPlayer 웹사이트 URL을 사용하여 프로그램 녹화
- `get_iplayer "프로그램_이름" --subtitles-only` : 검색 결과를 기반으로 프로그램 자막만 다운로드
- `get_iplayer "프로그램_이름" -g --subtitles` : 프로그램 검색 후, 녹화 및 자막도 함께 다운로드
- `get_iplayer -h` : 도움말 표시

## Links
- More information: https://github.com/get-iplayer/get_iplayer/wiki/manpage
