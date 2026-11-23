# pm list packages

Android 기기에 설치되었거나 알려진 패키지 목록 조회.

## Usage
- `pm list packages` : 설치된 모든 패키지 목록 표시
- `pm list packages -f` : 모든 패키지와 해당 APK 파일(file) 경로 표시
- `pm list packages -d` : 비활성화된(disabled) 패키지만 표시
- `pm list packages -e` : 활성화된(enabled) 패키지만 표시
- `pm list packages -s` : 시스템(system) 패키지만 표시
- `pm list packages -3` : 사용자 설치([3]rd-party) 패키지만 표시
- `pm list packages -i` : 각 패키지의 설치 관리자(installer) 정보 표시

## Links
- More information: https://developer.android.com/tools/adb#pm
