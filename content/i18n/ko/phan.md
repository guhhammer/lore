# phan

PHP용 정적 분석 도구.

## Usage
- `phan --init` : 현재 디렉터리에 `.phan/config.php` 생성
- `phan --init --init-level 1..5` : 특정 레벨을 사용하여 Phan 구성 파일 생성 (1이 가장 엄격하고 5가 가장 덜 엄격함)
- `phan` : 현재 디렉터리 분석
- `phan --directory 경로/대상/폴더 --directory 경로/대상/다른_폴더 ...` : 하나 이상의 디렉터리 분석
- `phan --config-file 경로/대상/config.php` : 구성 파일 지정 (기본값은 `.phan/config.php`)
- `phan --output-mode text|verbose|json|csv|codeclimate|checkstyle|pylint|html` : 출력 모드 지정
- `phan --processes 프로세스_수` : 병렬 프로세스 수 지정

## Links
- More information: https://github.com/phan/phan
