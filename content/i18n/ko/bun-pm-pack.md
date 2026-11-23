# bun pm pack

npm에 배포될 파일들을 포함한 `.tgz` 아카이브를 생성 (same behavior as `npm pack`).

## Usage
- `bun pm pack` : 현재 워크스페이스에서 tarball 생성
- `bun pm pack --dry-run` : 실제 디스크에 tarball을 생성하지 않고 모든 단계를 실행 (dry-run)
- `bun pm pack --destination 경로/대상/디렉토리` : tarball을 특정 디렉토리에 저장
- `bun pm pack --filename 파일이름` : tarball 파일명을 지정
- `bun pm pack --ignore-scripts` : prepack, postpack, prepare 스크립트 실행 건너뛰기
- `bun pm pack --gzip-level 5` : gzip 압축 단계 설정 (0-9, 기본: 9)
- `bun pm pack --quiet` : tarball 파일명만 출력하고 상세 로그 생략

## Links
- More information: https://bun.com/docs/pm/cli/pm#pack
