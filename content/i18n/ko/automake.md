# automake

GNU 표준을 기반으로 소프트웨어 프로젝트의 Makefile을 자동으로 생성하는 도구.

## Usage
- `automake` : `Makefile.am` 수정 후 Makefile 재생성
- `automake --foreign` : 비 GNU 프로젝트용 `Makefile.in` 생성 (foreign 모드)
- `automake -v` : 디버깅을 위한 상세 출력 활성화
- `automake -a` : 필요한 표준 파일 (INSTALL, COPYING, depcomp 등) 자동 추가
- `automake --help` : 도움말 표시
