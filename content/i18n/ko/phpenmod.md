# phpenmod

Debian 계열 OS에서 PHP 확장을 활성화.

## Usage
- `sudo phpenmod json` : 모든 PHP 버전의 모든 SAPI에 대해 JSON 확장 활성화
- `sudo phpenmod -v 7.3 -s cli json` : PHP 7.3의 cli SAPI에 대해 JSON 확장 활성화

## Links
- More information: https://salsa.debian.org/php-team/php-defaults
