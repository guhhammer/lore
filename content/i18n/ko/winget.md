# winget

윈도우 패키지 매니저.

## Usage
- `winget add 패키지` : 패키지 설치
- `winget rm 패키지` : 패키지 제거 (참고: `remove` 대신 `uninstall` 을 사용할 수 있음)
- `winget show 패키지` : 패키지에 대한 정보 표시
- `winget search 패키지` : 패키지 검색
- `winget upgrade -r` : 모든 패키지를 최신 버전으로 업그레이드
- `winget ls -s winget` : 관리할 수 있는 모든 패키지 나열
- `winget import|export --import-file|--output 경로\대상\파일` : 파일에서 패키지 가져오기 또는 설치된 패키지를 파일로 내보내기
- `winget validate 경로\대상\매니페스트` : winget 매니페스트 패키지 저장소에 PR을 제출하기 전에 유효성 검사

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
