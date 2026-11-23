# zipsplit

Zip 아카이브를 더 작은 Zip 아카이브로 분할.

## Usage
- `zipsplit 경로/대상/아카이브.zip` : zip 아카이브를 36000 바이트(36 MB) 이하의 부분으로 분할
- `zipsplit -n 크기 경로/대상/아카이브.zip` : 지정된 바이트 n 수를 부분 제한으로 사용
- `zipsplit -p -n 크기 경로/대상/아카이브.zip` : 각 부분 생성 사이에 p 일시 정지
- `zipsplit -b 경로/대상/출력_폴더 -n 크기 경로/대상/아카이브.zip` : 분할된 작은 Zip 아카이브를 지정된 디렉토리에 출력

## Links
- More information: https://manned.org/zipsplit
