# bun pm pkg

`get`, `set`, `delete`, `fix` 작업을 통해 `package.json` 데이터를 관리.

## Usage
- `bun pm pkg get` : `package.json`의 모든 속성 조회
- `bun pm pkg get 속성` : 특정 속성 하나 조회
- `bun pm pkg get 속성1 속성2 속성3 ...` : 여러 속성 조회
- `bun pm pkg get 속성.attribute` : 중첩된 속성 조회
- `bun pm pkg set 속성="값"` : 속성 설정
- `bun pm pkg delete 속성` : 속성 삭제
- `bun pm pkg fix` : `package.json`의 일반적인 문제를 자동으로 수정

## Links
- More information: https://bun.com/docs/pm/cli/pm#pkg
