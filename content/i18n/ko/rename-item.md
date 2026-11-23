# rename-item

항목의 이름을 변경하는 PowerShell 명령어.

## Usage
- `Rename-Item -Path "경로\대상\파일" -NewName "새로운_파일_이름"` : 파일 이름 변경
- `Rename-Item -Path "경로\대상\디렉터리" -NewName "새로운_디렉터리_이름"` : 디렉터리 이름 변경
- `Rename-Item -Path "경로\대상\파일" -NewName "경로\대상\새로운_파일_이름"` : 파일 이름 변경 및 이동
- `Rename-Item -Path "경로\대상\파일" -NewName "새로운_파일_이름" -Force` : 강제로 파일 이름 변경
- `Rename-Item -Path "경로\대상\파일" -NewName "새로운_파일_이름" -Confirm` : 파일 이름 변경 전에 확인 메시지 표시
