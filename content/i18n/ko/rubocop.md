# rubocop

Ruby 파일을 린트합니다.

## Usage
- `rubocop` : 현재 디렉토리의 모든 파일 확인(하위 디렉토리 포함)
- `rubocop 경로/대상/파일_또는_폴더1 경로/대상/파일_또는_폴더2 ...` : 하나 이상의 특정 파일 또는 디렉토리 확인
- `rubocop --out 경로/대상/파일` : 출력 결과를 파일에 저장
- `rubocop --show-cops` : cop(린터 규칙) 목록 보기
- `rubocop --except cop1 cop2 ...` : 특정 cop 제외
- `rubocop --only cop1 cop2 ...` : 지정된 cop만 실행
- `rubocop --auto-correct` : 파일 자동 수정(실험적 기능)

## Links
- More information: https://docs.rubocop.org/rubocop/latest/usage/cli_reference.html
