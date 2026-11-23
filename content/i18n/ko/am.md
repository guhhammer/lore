# am

Android 활동 관리자.

## Usage
- `am start -n com.android.settings/.Settings` : 특정 패키지 이름(name)과 컴포넌트를 지정해 액티비티 시작
- `am start -a android.intent.action.VIEW -d tel:123` : 특정 인텐트 액션(action)을 실행하고 데이터(data) 전달
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : 특정 액션(action)과 카테고리(category)에 일치하는 액티비티 시작
- `am to-uri -a android.intent.action.VIEW -d tel:123` : 인텐트를 URI 형식으로 변환
- `am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : 에뮬레이터 또는 기기에서 홈 액티비티 시작

## Links
- More information: https://developer.android.com/tools/adb#am
