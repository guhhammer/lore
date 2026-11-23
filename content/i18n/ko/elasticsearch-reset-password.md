# elasticsearch-reset-password

네이티브 영역 사용자 및 기본 제공 사용자들의 비밀번호를 재설정하는 도구.

## Usage
- `elasticsearch-reset-password -u 사용자` : 사용자 비밀번호를 자동 생성 값으로 재설정 후, 콘솔에 출력
- `elasticsearch-reset-password -u 사용자 -i` : 네이티브 사용자 비밀번호를 대화형으로 재설정
- `elasticsearch-reset-password --url 호스트:포트 -u 사용자 -i` : 지정한 Elasticsearch 노드 URL에서 사용자 비밀번호를 대화형으로 재설정
