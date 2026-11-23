# ncc

Node.js 애플리케이션을 단일 파일로 컴파일. TypeScript, 바이너리 애드온 및 동적 require를 지원.

## Usage
- `ncc build 경로/대상/파일.js` : node.js 애플리케이션 번들링
- `ncc build -m 경로/대상/파일.js` : node.js 애플리케이션을 번들링하고 축소
- `ncc build -s 경로/대상/파일.js` : node.js 애플리케이션을 번들링하고 축소하며 소스 맵 생성
- `ncc build -w 경로/대상/파일.js` : 소스 파일 변경 시 자동으로 다시 컴파일
- `ncc run 경로/대상/파일.js` : node.js 애플리케이션을 임시 디렉토리에 번들링하고 테스트를 위해 실행
- `ncc clean cache` : `ncc` 캐시 삭제

## Links
- More information: https://github.com/vercel/ncc#usage
