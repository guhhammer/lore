# expac

pacman 기반 유틸리티를 위해 printf와 유사한 형식 지정 기능을 제공하는 alpm 데이터베이스 정보 추출 도구.

## Usage
- `expac -S '%D' 패키지` : 패키지의 의존성 목록 표시
- `expac -S "%o" 패키지` : 패키지의 선택적 의존성 목록 표시
- `expac -S -H M '%k\t%n' 패키지1 패키지2 ...` : 패키지의 다운로드 크기를 MiB 단위로 표시
- `expac -S -H M '%k\t%n' $(pacman -Qqu) | sort -sh` : 업그레이드 대상 패키지와 다운로드 크기 표시
- `expac -d '\n\n' -l '\n\t' -Q '%n\n\t%O' $(pacman -Qeq)` : 명시적으로 설치한 패키지와 선택적 의존성 목록 표시

## Links
- More information: https://github.com/falconindy/expac#name
