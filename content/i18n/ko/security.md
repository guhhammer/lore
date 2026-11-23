# security

키체인, 키, 인증서 및 보안 프레임워크를 관리합니다.

## Usage
- `security list-keychains` : 사용 가능한 모든 키체인 나열
- `security delete-keychain 경로/대상/파일.keychain` : 특정 키체인 삭제
- `security create-keychain -p 비밀번호 경로/대상/파일.keychain` : 키체인 생성
- `security set-identity-preference -s url|호스트명|서비스 -c "일반_이름" 경로/대상/파일.keychain` : 인증서를 웹사이트 또는 s서비스에서 사용하도록 c일반 이름으로 설정 (동일한 일반 이름을 가진 인증서가 여러 개 있는 경우 실패)
- `security add-certificates -k 파일.keychain 경로/대상/인증서_파일.pem` : 파일에서 k키체인으로 인증서 추가 (-k가 지정되지 않으면 기본 키체인이 사용됨)
- `security add-trusted-cert -k 경로/대상/사용자-키체인.keychain-db 경로/대상/ca-인증서_파일.pem` : 사용자별 신뢰 설정에 CA 인증서 추가
- `security remove-trusted-cert 경로/대상/ca-인증서_파일.pem` : 사용자별 신뢰 설정에서 CA 인증서 제거

## Links
- More information: https://keith.github.io/xcode-man-pages/security.1.html
