# reg add

레지스트리에 새 키와 값을 추가.

## Usage
- `reg add 키_이름` : 새 레지스트리 키 추가
- `reg add 키_이름 /v 값` : 특정 키 아래에 새 value 추가
- `reg add 키_이름 /d 데이터` : 특정 data로 새 값 추가
- `reg add 키_이름 /t REG_SZ|MULTI_SZ|DWORD_BIG_ENDIAN|DWORD|BINARY|DWORD_LITTLE_ENDIAN|LINK|FULL_RESOURCE_DESCRIPTOR|EXPAND_SZ` : 특정 데이터 type으로 키에 새 값 추가
- `reg add 키_이름 /f` : forcefully (프롬프트 없이) 기존 레지스트리 값 덮어쓰기
