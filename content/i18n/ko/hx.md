# hx

Helix는 현대적인 텍스트 에디터로, 텍스트 조작을 위한 여러 모드를 제공. `<i>`를 누르면, 입력 모드로 전환되고, `<Esc>`를 누르면 일반 모드로 전환되어, Helix 명령을 사용할 수 있음.

## Usage
- `hx 경로/대상/파일` : 파일 열기
- `hx --vsplit 경로/대상/파일1 경로/대상/파일2 ...` : 여러 파일을 나란히(분할 화면) 열기
- `hx --tutor` : helix 튜토리얼 실행 (또는 Helix 실행 중 `<Esc>` 후, `<:>tutor<Enter>` 입력)
- `<:>theme 테마_이름` : helix 테마 변경
- `<:>wq<Enter>` : 저장 후 종료
- `<:>q!<Enter>` : 저장하지 않고 강제 종료
- `<u>` : 마지막 작업 되돌리기
- `</>검색_패턴<Enter>` : 파일 내에서 패턴 검색 (`<n>`/`<N>`으로 다음/이전 결과 이동)

## Links
- More information: https://manned.org/man/debian-forky/hx
