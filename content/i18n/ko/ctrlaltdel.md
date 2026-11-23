# ctrlaltdel

`<Ctrl Alt Del>` 키 조합을 눌렀을 때의 동작을 제어하는 도구.

## Usage
- `ctrlaltdel` : 현재 설정 확인
- `sudo ctrlaltdel hard` : `<Ctrl Alt Del>`을 즉시 재부팅하도록 설정 (준비 없이)
- `sudo ctrlaltdel soft` : `<Ctrl Alt Del>`을 "일반적으로" 재부팅하도록 설정, 프로세스 종료 기회를 제공 (PID1에 `SIGINT` 전송)

## Links
- More information: https://manned.org/ctrlaltdel
