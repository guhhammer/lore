# safe

HashiCorp Vault와 상호작용.

## Usage
- `safe target vault_주소 타겟_이름` : 안전한 타겟 추가
- `safe auth 인증_토큰` : 인증 토큰을 사용하여 Vault 서버에 CLI 클라이언트 인증
- `safe env` : 현재 타겟을 설명하는 환경 변수 출력
- `safe tree 경로` : 주어진 경로에 대한 모든 접근 가능한 키의 트리 계층 구조 표시
- `safe move 기존/경로/대상/비밀 새로운/경로/대상/비밀` : 비밀을 한 경로에서 다른 경로로 이동
- `safe ssh 2048 경로/대상/비밀` : 새로운 2048비트 SSH 키 쌍 생성 및 저장
- `safe set 경로/대상/비밀 키=값` : 비민감 키를 비밀에 설정
- `safe gen 경로/대상/비밀 키` : 자동 생성된 비밀번호를 비밀에 설정

## Links
- More information: https://github.com/egen/safe
