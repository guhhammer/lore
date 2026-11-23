# vitest

Vite를 기반으로 빠르고 현대적인 테스트 프레임워크로, 원활한 통합, TypeScript 지원 및 Jest 호환 API를 제공하여 단위, 통합 및 스냅샷 테스트를 지원.

## Usage
- `vitest run` : 사용 가능한 모든 테스트 실행
- `vitest run 경로/대상/파일1 경로/대상/파일2 ...` : 지정한 파일의 테스트 스위트 실행
- `vitest run regex1 regex2 ...` : 현재 디렉터리와 하위 디렉터리에서 경로가, 지정한 정규 표현식(`regex`)과 일치하는 파일의 테스트 스위트 실행
- `vitest run -t regex` : 이름이 지정한 정규 표현식(`regex`)과 일치하는 테스트 실행
- `vitest` : 파일의 변경 사항을 감시하고 관련 테스트를 자동으로 다시 실행
- `vitest run --coverage` : 코드 커버리지를 측정하며 테스트 실행
- `vitest run --bail=1` : 모든 테스트를 실행하지만 첫 번째 테스트 실패 시 즉시 중단
- `vitest --help` : 도움말 표시

## Links
- More information: https://vitest.dev/guide/cli.html
