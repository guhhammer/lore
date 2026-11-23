# skate

간단하고 강력한 키-값 저장소.

## Usage
- `skate set "키" "값"` : 기본 데이터베이스에 키와 값을 저장
- `skate list` : 기본 데이터베이스에 저장된 키 표시
- `skate delete "키"` : 기본 데이터베이스에서 키와 값 삭제
- `skate set "키"@"데이터베이스_이름" "값"` : 새로운 데이터베이스에 키와 값을 생성
- `skate list @"데이터베이스_이름"` : 기본이 아닌 데이터베이스에 저장된 키 표시
- `skate delete "키"@"데이터베이스_이름"` : 특정 데이터베이스에서 키와 값 삭제
- `skate list-dbs` : 사용 가능한 데이터베이스 표시
- `skate reset @"데이터베이스_이름"` : 로컬 데이터베이스 삭제 및 Charm Cloud에서 새 복사본 가져오기

## Links
- More information: https://manned.org/skate
