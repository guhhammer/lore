# infection

PHP용 코드 변경 테스팅 프레임워크.

## Usage
- `infection` : 구성 파일을 사용하여 코드를 분석 (또는 존재하지 않는 경우 새롭게 생성)
- `infection -j 스레드_수` : 특정 수의 스레드를 사용
- `infection --min-msi 백분율` : 최소 MSI(Mutation Score Indicator)를 지정
- `infection --min-covered-msi 백분율` : 최소 적용 코드 MSI를 지정
- `infection --test-framework phpunit|phpspec` : 특정 테스트 프레임워크를 사용 (기본값은 PHPUnit)
- `infection --only-covered` : 테스트에 포함된 코드 줄만 변경
- `infection -s` : 적용된 코드 변경을 주는 부분을 표시
- `infection --log-verbosity default|all|none` : 로그 상세 수준을 지정

## Links
- More information: https://infection.github.io/guide/command-line-options.html
