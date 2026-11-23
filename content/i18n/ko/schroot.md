# schroot

다른 루트 디렉터리로 명령을 실행하거나 대화형 셸을 시작합니다. `chroot`보다 더 커스터마이즈 가능합니다.

## Usage
- `schroot --list` : 사용 가능한 chroot 목록 나열
- `schroot --chroot chroot 명령` : 특정 chroot에서 명령 실행
- `schroot --chroot chroot 명령 -- 명령_옵션` : 특정 chroot에서 옵션과 함께 명령 실행
- `schroot --all 명령` : 모든 사용 가능한 chroot에서 명령 실행
- `schroot --chroot chroot --user 사용자` : 특정 사용자로 특정 chroot 내에서 대화형 셸 시작
- `schroot --begin-session --chroot chroot` : 새 세션 시작 (고유한 세션 ID가 `stdout`에 반환됨)
- `schroot --run-session --chroot 세션_id` : 기존 세션에 연결
- `schroot --end-session --chroot 세션_id` : 기존 세션 종료

## Links
- More information: https://wiki.debian.org/Schroot
