# odpscmd auth

ODPS (Open Data Processing Service)에서 사용자 권한 관리.

## Usage
- `add user 사용자명;` : [대화형] 현재 프로젝트에 사용자 추가
- `grant 작업_목록 on 객체_유형 객체_이름 to user 사용자명;` : [대화형] 사용자에게 권한 집합 부여
- `show grants for 사용자명;` : [대화형] 사용자의 권한 보기
- `create role 역할_이름;` : [대화형] 사용자 역할 생성
- `grant 작업_목록 on 객체_유형 객체_이름 to role 역할_이름;` : [대화형] 역할에 권한 집합 부여
- `desc role 역할_이름;` : [대화형] 역할의 권한 설명
- `grant 역할_이름 to 사용자명;` : [대화형] 사용자에게 역할 부여

## Links
- More information: https://www.alibabacloud.com/help/en/maxcompute/user-guide/maxcompute-client
