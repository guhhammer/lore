# apkleaks

APK 파일에서 URI, 엔드포인트, 비밀을 노출. 참고: APKLeaks는 `jadx` 디스어셈블러를 사용하여 APK 파일을 디컴파일.

## Usage
- `apkleaks -f 경로/대상/파일.apk` : APK 파일(file)에서 URI, 엔드포인트, 비밀을 스캔
- `apkleaks -f 경로/대상/파일.apk -o 경로/대상/출력파일.txt` : 출력(output)을 스캔하여 특정 파일에 저장
- `apkleaks -f 경로/대상/파일.apk -a "--threads-count 5 --deobf"` : `jadx` 디스어셈블러 인수(arguments) 전달

## Links
- More information: https://github.com/dwisiswant0/apkleaks
