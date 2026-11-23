# xdg-user-dirs-update

XDG 사용자 디렉터리 업데이트.

## Usage
- `xdg-user-dirs-update --set DESKTOP "/경로/대상/폴더"` : XDG의 DESKTOP 디렉터리를 지정한 디렉터리로 변경 (절대 경로여야 함)
- `xdg-user-dirs-update --dummy-output "경로/대상/실행_파일" --set xdg_사용자_디렉터리 "/경로/대상/폴더"` : 결과를 `user-dirs.dirs` 파일 대신 지정한 실행 파일에 기록

## Links
- More information: https://manned.org/xdg-user-dirs-update
