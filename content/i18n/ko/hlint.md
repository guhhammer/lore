# hlint

Haskell 코드에 대한 개선 사항을 제안합니다.

## Usage
- `hlint 경로/대상/파일 options` : 주어진 파일에 대한 제안 사항 표시
- `hlint 경로/대상/폴더 -r` : 모든 Haskell 파일을 검사하고 보고서 생성
- `hlint 경로/대상/파일 --refactor` : 대부분의 제안을 자동으로 적용
- `hlint 경로/대상/파일 --refactor-options` : 추가 옵션 표시
- `hlint 경로/대상/파일 --default >.hlint.yaml` : 모든 미해결 힌트를 무시하는 설정 파일 생성

## Links
- More information: https://hackage.haskell.org/package/hlint
