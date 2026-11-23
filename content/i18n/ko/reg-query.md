# reg query

레지스트리의 키와 하위 키의 값을 표시.

## Usage
- `reg query 키_이름` : 키의 모든 값 표시
- `reg query 키_이름 /v 값` : 키의 특정 값 표시
- `reg query 키_이름 /s` : 키와 하위 키의 모든 값 표시
- `reg query 키_이름 /f "검색_패턴"` : 특정 패턴과 일치하는 키와 값을 [검색]
- `reg query 키_이름 /t REG_SZ|MULTI_SZ|EXPAND_SZ|DWORD|BINARY|NONE` : 지정된 데이터 [형식]과 일치하는 키의 값 표시
- `reg query 키_이름 /d` : 데이터에서만 검색
- `reg query 키_이름 /f "검색_패턴" /k` : 키 이름에서만 검색
- `reg query 키_이름 /c /e` : [대소문자] 구분하여 [정확히] 일치하는 값 검색
