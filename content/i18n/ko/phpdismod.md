# phpdismod

Debian 기반 운영 체제에서 PHP 확장을 비활성화합니다.

## Usage
- `sudo phpdismod json` : 모든 PHP 버전의 모든 SAPI에서 JSON 확장 비활성화
- `sudo phpdismod -v 7.3 -s cli json` : PHP 7.3 버전의 cli SAPI에서 JSON 확장 비활성화

## Links
- More information: https://salsa.debian.org/php-team/php-defaults
