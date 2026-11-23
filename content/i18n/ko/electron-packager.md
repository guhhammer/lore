# electron-packager

Windows, Linux 및 macOS용 Electron 앱 실행 파일을 빌드. 애플리케이션 디렉터리에 유효한 package.json이 필요.

## Usage
- `electron-packager "경로/대상/앱" "애플리케이션"` : 현재 아키텍처 및 플랫폼에 대한 애플리케이션을 패키징
- `electron-packager "경로/대상/앱" "애플리케이션" --all` : 모든 아키텍처와 플랫폼에 대한 애플리케이션을 패키징
- `electron-packager "경로/대상/앱" "애플리케이션" --platform="linux" --arch="x64"` : 64비트 Linux용 애플리케이션을 패키징
- `electron-packager "경로/대상/앱" "애플리케이션" --platform="darwin" --arch="arm64"` : ARM macOS용 애플리케이션을 패키징

## Links
- More information: https://github.com/electron/packager
