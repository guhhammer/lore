# ts-node

TypeScript 코드를 컴파일 없이 직접 실행.

## Usage
- `ts-node 경로/대상/파일.ts` : typeScript 파일을 컴파일 없이 실행 (Node + `tsc`)
- `ts-node --skipProject 경로/대상/파일.ts` : `tsconfig.json`을 로드하지 않고 TypeScript 파일 실행
- `ts-node -e 'console.log("Hello World")'` : 리터럴로 전달된 TypeScript 코드 평가
- `ts-node --script-mode 경로/대상/파일.ts` : 스크립트 모드로 TypeScript 파일 실행
- `ts-node -T 경로/대상/파일.ts` : typeScript 파일을 실행하지 않고 JavaScript로 트랜스파일
- `ts-node -h` : 도움말 표시

## Links
- More information: https://typestrong.org/ts-node/docs/options/
