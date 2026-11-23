# pm

Android 패키지 관리자 도구.

## Usage
- `pm list packages` : 설치된 패키지 목록 출력
- `pm install /경로/대상/앱.apk` : 지정한 경로의 app 패키지 설치
- `pm uninstall 패키지` : 기기에서 패키지 삭제
- `pm clear 패키지` : 특정 패키지의 모든 앱 데이터 삭제
- `pm enable 패키지_또는_클래스` : 패키지 또는 컴포넌트 활성화
- `pm disable-user 패키지_또는_클래스` : 패키지 또는 컴포넌트 비활성화
- `pm grant 패키지 android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : 애플리케이션에 권한 부여
- `pm revoke 패키지 android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : 애플리케이션에서 권한 제거

## Links
- More information: https://developer.android.com/tools/adb#pm
