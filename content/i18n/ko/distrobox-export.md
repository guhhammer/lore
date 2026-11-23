# distrobox-export

컨테이너에서 호스트 OS로 앱/서비스/바이너리를 내보냅니다.

## Usage
- `distrobox-export -a 패키지 -ef "--foreground"` : 컨테이너에서 호스트로 앱 내보내기 (데스크톱 항목/아이콘이 호스트 시스템의 응용 프로그램 목록에 나타납니다)
- `distrobox-export -b 경로/대상/바이너리 -ep 경로/대상/호스트_바이너리` : 컨테이너에서 호스트로 바이너리 내보내기
- `distrobox-export -b 경로/대상/바이너리 -ep 경로/대상/내보내기` : 컨테이너에서 호스트로 바이너리 내보내기 (예: `$HOME/.local/bin`)
- `distrobox-export --service 패키지 -ef "--allow-newer-config" -S` : 컨테이너에서 호스트로 서비스 내보내기 (`--sudo`는 해당 서비스를 컨테이너 내에서 루트 권한으로 실행)
- `distrobox-export -a 패키지 -d` : 내보낸 응용 프로그램 제거/삭제

## Links
- More information: https://distrobox.it/usage/distrobox-export/
