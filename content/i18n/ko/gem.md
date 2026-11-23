# gem

Ruby 프로그래밍 언어용 패키지 관리자.

## Usage
- `gem search 정규_표현식 -a` : 원격 gem을 검색하고 사용 가능한 모든 버전을 표시
- `gem install 젬_이름` : 최신 버전의 gem을 설치
- `gem install 젬_이름 -v 1.0.0` : 특정 버전의 gem을 설치
- `gem install 젬_이름 -v '~> 1.0'` : 일치하는 최신 (SemVer) 버전의 gem을 설치
- `gem update 젬_이름` : gem 업데이트
- `gem list` : 모든 로컬 gem을 나열
- `gem uninstall 젬_이름` : gem 제거
- `gem uninstall 젬_이름 -v 1.0.0` : 특정 버전의 gem을 제거

## Links
- More information: https://guides.rubygems.org/command-reference/
