# spctl

보안 평가 정책 하위 시스템 관리. macOS에서 Gatekeeper를 관리하는 도구.

## Usage
- `spctl --master-disable` : gatekeeper 비활성화
- `spctl --add --label 규칙_이름 경로/대상/파일` : 애플리케이션 실행을 허용하는 규칙 추가 (규칙의 라벨 지정은 선택 사항)
- `spctl --master-enable` : gatekeeper 활성화
- `spctl --list` : 시스템의 모든 규칙 나열

## Links
- More information: https://keith.github.io/xcode-man-pages/spctl.8.html
