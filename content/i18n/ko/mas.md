# mas

Mac App Store용 명령줄 인터페이스.

## Usage
- `mas signin "user@example.com"` : mac App Store에 처음으로 로그인
- `mas list` : 설치된 모든 애플리케이션과 그 제품 식별자 표시
- `mas search "애플리케이션" --price` : 애플리케이션 검색 및 결과와 함께 가격 표시
- `mas install 숫자_제품_id` : 정확한 숫자 ID를 사용하여 애플리케이션 설치 또는 업데이트
- `mas lucky "검색어"` : 검색에서 반환되는 첫 번째 애플리케이션 설치
- `mas outdated` : 업데이트가 필요한 모든 구버전 앱 나열
- `mas upgrade` : 모든 대기 중인 업데이트 설치
- `mas upgrade "숫자_제품_id"` : 특정 애플리케이션 업그레이드

## Links
- More information: https://github.com/mas-cli/mas
