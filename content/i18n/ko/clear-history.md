# clear-history

PowerShell 세션 명령 기록을 삭제함.

## Usage
- `Clear-History` : 현재 세션의 모든 명령 기록 삭제
- `Clear-History -CommandLine "명령어"` : 지정한 명령의 기록 삭제
- `Clear-History -CommandLine "명령어1", "명령어2", ...` : 여러 명령의 기록 삭제
- `Clear-History -Id 아이디_숫자` : 지정한 ID의 기록 삭제
- `Clear-History -Id 아이디1, 아이디2, ...` : 여러 ID의 기록 삭제
- `Clear-History -Id (시작_아이디..종료_아이디)` : 지정한 ID 범위의 기록 삭제
- `Clear-History -WhatIf` : 삭제될 항목만 표시
- `Clear-History -Confirm` : 삭제 전에 확인 메시지 표시
