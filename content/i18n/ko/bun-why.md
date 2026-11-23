# bun why

특정 패키지가 왜 설치되었는지 의존성 체인을 통해 설명.

## Usage
- `bun why 패키지_이름` : 특정 패키지가 설치된 이유 확인
- `bun why "패턴"` : 패턴에 일치하는 모든 패키지가 설치된 이유 확인
- `bun why "@조직/*"` : 특정 조직의 패키지가 설치된 이유 확인
- `bun why 패키지_이름 --top` : 최상위 의존성만 표시
- `bun why 패키지_이름 --depth 번호` : 의존성 트리 깊이 제한

## Links
- More information: https://bun.com/docs/pm/cli/why
