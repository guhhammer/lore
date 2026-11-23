# rails db

Ruby on Rails의 다양한 데이터베이스 관련 하위 명령어.

## Usage
- `rails db:setup` : 데이터베이스 생성, 스키마 로드 및 시드 데이터로 초기화
- `rails db` : 데이터베이스 콘솔에 접근
- `rails db:create` : 현재 환경에 정의된 데이터베이스 생성
- `rails db:drop` : 현재 환경에 정의된 데이터베이스 삭제
- `rails db:migrate` : 보류 중인 마이그레이션 실행
- `rails db:migrate:status` : 각 마이그레이션 파일의 상태 보기
- `rails db:rollback` : 마지막 마이그레이션 롤백
- `rails db:seed` : `db/seeds.rb`에 정의된 데이터로 현재 데이터베이스 채우기

## Links
- More information: https://guides.rubyonrails.org/active_record_migrations.html
