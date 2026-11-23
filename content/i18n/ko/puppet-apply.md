# puppet apply

Puppet 매니페스트를 로컬에서 적용.

## Usage
- `puppet apply 경로/대상/매니페스트` : 매니페스트 적용
- `puppet apply --execute 코드` : puppet 코드 실행
- `puppet apply --modulepath 경로/대상/폴더 --hiera_config 경로/대상/파일 경로/대상/매니페스트` : 특정 모듈 및 Hiera 구성 파일 사용

## Links
- More information: https://github.com/puppetlabs/puppet/blob/main/references/man/apply.md
