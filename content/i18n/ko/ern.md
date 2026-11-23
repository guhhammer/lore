# ern

전극 네이티브 플랫폼 명령줄 클라이언트.

## Usage
- `ern create-miniapp 애플리케이션_이름` : 새로운 `ern` 애플리케이션(`MiniApp`)을 생성
- `ern run-ios|android` : iOS/Android 러너 애플리케이션에서 하나 이상의 `MiniApps`를 실행
- `ern create-container --miniapps /경로/대상/miniapp_디렉토리 --platform ios|android` : 전극 네이티브 컨테이너 만들기
- `ern publish-container --publisher maven --platform android --extra '{"groupId":"com.walmart.ern","artifactId":"quickstart"}'` : 전극 네이티브 컨테이너를 로컬 Maven 저장소에 저장
- `ern transform-container --platform ios --transformer xcframework` : iOS 컨테이너를 사전 컴파일된 바이너리 프레임워크로 변환
- `ern platform versions` : 설치된 모든 전극 Native 버전을 나열
- `ern platform config set logLevel trace|debug` : 로깅 수준을 설정

## Links
- More information: https://native.electrode.io/reference/index-6
