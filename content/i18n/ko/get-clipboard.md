# get-clipboard

클립보드의 내용을 가져오는 PowerShell 명령.

## Usage
- `Get-Clipboard` : 클립보드의 텍스트 가져오기
- `Get-Clipboard -TextFormatType Text|Html|Rtf` : 지정한 텍스트 형식으로 클립보드 내용 가져오기
- `Get-Clipboard -Raw` : 클립보드의 원본 내용 가져오기
- `Get-Clipboard -Format Image` : 클립보드의 이미지 가져오기
- `Get-Clipboard -Format FileDropList` : 파일 탐색기에서 복사한 파일 경로 가져오기
