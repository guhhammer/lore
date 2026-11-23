# spotify_player

공식 Spotify 애플리케이션의 모든 기능을 지원하는 TUI 기반 Spotify 클라이언트.

## Usage
- `spotify_player -d` : 백그라운드에서 음악을 재생하는 데몬 시작
- `spotify_player` : TUI 실행 (사용 가능한 데몬이 있으면 제어하고, 없으면 자체 클라이언트 시작)
- `spotify_player -t 테마_이름` : 지정한 테마 사용
- `spotify_player -c 경로/대상/디렉터리` : 지정한 디렉터리의 설정 파일 (`app.toml`, `keymap.toml`, `theme.toml`)
- `spotify_player like` : 현재 재생 중인 트랙에 좋아요 표시
- `<?>` : 키 바인딩 목록 표시

## Links
- More information: https://github.com/aome510/spotify-player#commands
