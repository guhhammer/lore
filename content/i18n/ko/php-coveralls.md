# php-coveralls

Coveralls를 위한 PHP 클라이언트.

## Usage
- `php-coveralls` : coveralls에 커버리지 정보를 전송
- `php-coveralls -r 경로/대상/폴더` : 특정 디렉토리에 대한 커버리지 정보를 Coveralls에 전송
- `php-coveralls -c 경로/대상/.coveralls.yml` : 특정 설정 파일을 사용하여 Coveralls에 커버리지 정보를 전송
- `php-coveralls -v` : 자세한 출력과 함께 Coveralls에 커버리지 정보를 전송
- `php-coveralls --exclude-no-stmt` : 실행 가능한 문장이 없는 소스 파일을 제외하고 Coveralls에 커버리지 정보를 전송
- `php-coveralls -e test|dev|prod` : 특정 환경 이름을 사용하여 Coveralls에 커버리지 정보를 전송
- `php-coveralls -x 경로/대상/첫번째_clover.xml --coverage_clover 경로/대상/두번째_clover.xml` : 여러 Coverage Clover XML 파일을 업로드하도록 지정
- `php-coveralls -o 경로/대상/coveralls-전송.json` : coveralls에 전송될 JSON을 특정 파일로 출력

## Links
- More information: https://php-coveralls.github.io/php-coveralls/#cli-options
