# carthage

Cocoa 애플리케이션을 위한 의존성 관리 도구.

## Usage
- `carthage update` : cartfile에 언급된 모든 의존성을 최신 버전으로 다운로드하고 빌드
- `carthage update --platform ios` : 의존성을 업데이트하되, iOS용으로만 빌드
- `carthage update --no-build` : 의존성을 업데이트하되, 빌드하지 않음
- `carthage bootstrap` : 현재 버전의 의존성을 다운로드하고 재빌드(업데이트하지 않음)
- `carthage build 의존성` : 특정 의존성을 재빌드

## Links
- More information: https://github.com/Carthage/Carthage
