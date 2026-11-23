# transmission-remote

`transmission-daemon` 및 `transmission`의 원격 제어 도구.

## Usage
- `transmission-remote 호스트명 -a 토렌트|url -w /경로/대상/다운로드_디렉토리` : 토렌트 파일 또는 마그넷 링크를 Transmission에 추가하고 지정한 디렉토리로 다운로드
- `transmission-remote 호스트명 -w /경로/대상/다운로드_디렉토리` : 기본 다운로드 디렉토리 변경
- `transmission-remote 호스트명 -l` : 모든 토렌트 나열
- `transmission-remote 호스트명 -t "1,2" -s -t 3 -S` : 토렌트 1과 2 시작, 토렌트 3 중지
- `transmission-remote 호스트명 -t 1 -r -t 2 -rad` : 토렌트 1과 2 제거, 토렌트 2의 로컬 데이터도 삭제
- `transmission-remote 호스트명 -t all -S` : 모든 토렌트 중지
- `transmission-remote 호스트명 -t "1-10,15-20" --move /경로/대상/새_디렉토리` : 토렌트 1-10 및 15-20을 새 디렉토리로 이동 (존재하지 않는 경우 생성됨)

## Links
- More information: https://manned.org/transmission-remote
