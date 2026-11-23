# xctool

Xcode 프로젝트 빌드.

## Usage
- `xctool -project YourProject.xcodeproj -scheme YourScheme build` : 워크스페이스 없이 단일 프로젝트 빌드
- `xctool -workspace YourWorkspace.xcworkspace -scheme YourScheme build` : 워크스페이스의 일부인 프로젝트 빌드
- `xctool -workspace YourWorkspace.xcworkspace -scheme YourScheme clean build test` : 정리하고, 빌드하고, 모든 테스트 실행

## Links
- More information: https://github.com/facebookarchive/xctool#usage
