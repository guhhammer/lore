# needrestart

라이브러리 업그레이드 후 다시 시작해야 하는 데몬 확인.

## Usage
- `needrestart` : 오래된 프로세스 나열
- `sudo needrestart` : 상호작용 모드로 서비스 다시 시작
- `needrestart -v|-q` : v자세히 또는 q조용히 모드에서 오래된 프로세스 나열
- `needrestart -k` : k커널이 오래되었는지 확인
- `needrestart -w` : CPU 마이크로코드가 오래되었는지 확인
- `needrestart -b` : b배치 모드에서 오래된 프로세스 나열
- `needrestart -c 경로/대상/설정` : 특정 c구성 파일을 사용하여 오래된 프로세스 나열
- `needrestart --help` : 도움말 표시

## Links
- More information: https://manned.org/needrestart
