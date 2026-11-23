# monop

.NET 어셈블리 내의 타입 및 메서드 시그니처를 찾아 표시.

## Usage
- `monop System.String` : .NET Framework의 내장 타입 구조 표시
- `monop -r:경로/대상/어셈블리.exe` : 어셈블리 내 타입 나열
- `monop -r:경로/대상/어셈블리.dll 네임스페이스.경로.대상.타입` : 특정 어셈블리의 타입 구조 표시
- `monop -r:경로/대상/어셈블리.dll -d 네임스페이스.경로.대상.타입` : 지정된 타입에 정의된 멤버만 표시
- `monop -r:경로/대상/어셈블리.dll -p 네임스페이스.경로.대상.타입` : 비공개 멤버 표시
- `monop -r:경로/대상/어셈블리.dll -f 네임스페이스.경로.대상.타입` : 사용되지 않는 멤버 숨기기
- `monop -r:경로/대상/어셈브리.dll --refs` : 특정 어셈블리가 참조하는 다른 어셈블리 나열

## Links
- More information: https://manned.org/monop
