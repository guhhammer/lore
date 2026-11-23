# set-clipboard

클립보드에 내용을 설정하는 PowerShell 명령어.

## Usage
- `Set-Clipboard -Value "텍스트"` : 텍스트를 클립보드에 복사
- `Set-Clipboard -Value @("텍스트 1", "텍스트 2", "텍스트 3")` : 여러 텍스트를 줄바꿈으로 구분하여 복사
- `Set-Clipboard -Path "경로\대상\파일_또는_디렉토리"` : 파일 또는 디렉터리를 클립보드에 복사
- `Set-Clipboard -Path "경로\대상\파일1","경로\대상\파일2","경로\대상\파일3"` : 여러 파일 복사
- `Set-Clipboard ""` : 클립보드 비우기
