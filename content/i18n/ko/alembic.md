# alembic

SQLAlchemy를 위한 데이터베이스 마이그레이션 도구.

## Usage
- `alembic init 경로/대상/디렉터리` : 프로젝트에서 Alembic을 초기화
- `alembic revision --autogenerate -m "메시지"` : 자동생성 기능을 사용해 새로운 마이그레이션 스크립트를 생성
- `alembic upgrade head` : 데이터베이스를 최신 리비전으로 업그레이드
- `alembic downgrade -1` : 데이터베이스를 한 단계 이전 리비전 상태로 되돌림
- `alembic history` : 마이그레이션 기록을 표시

## Links
- More information: https://manned.org/alembic
