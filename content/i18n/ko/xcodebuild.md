# xcodebuild

Xcode 프로젝트 빌드.

## Usage
- `xcodebuild -workspace 워크스페이스_이름.workspace -scheme 스킴_이름 -configuration 구성_이름 clean build SYMROOT=SYMROOT_경로` : 워크스페이스 빌드
- `xcodebuild -target 대상_이름 -configuration 구성_이름 clean build SYMROOT=SYMROOT_경로` : 프로젝트 빌드
- `xcodebuild -showsdks` : SDK 표시

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
