# php-cs-fixer

PHP 코딩 스타일 자동 수정 도구.

## Usage
- `php-cs-fixer fix` : 현재 디렉토리에서 코드 스타일 수정 실행
- `php-cs-fixer fix 경로/대상/폴더` : 특정 디렉토리에서 코드 스타일 수정 실행
- `php-cs-fixer fix --dry-run` : 변경 사항을 적용하지 않고 코드 스타일 검사 실행
- `php-cs-fixer fix --rules=규칙들` : 특정 규칙을 사용하여 코드 스타일 수정 실행
- `php-cs-fixer fix --verbose` : 적용된 규칙 표시
- `php-cs-fixer fix --format=txt|json|xml|checkstyle|junit|gitlab` : 다른 형식으로 결과 출력
- `php-cs-fixer list-files` : 수정이 필요한 파일 표시
- `php-cs-fixer describe 규칙` : 규칙 또는 규칙 세트 설명

## Links
- More information: https://github.com/PHP-CS-Fixer/PHP-CS-Fixer
