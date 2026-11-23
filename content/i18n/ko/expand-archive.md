# expand-archive

PowerShell에서 압축 아카이브 파일의 내용을 추출하는 cmdlet.

## Usage
- `Expand-Archive -Path 경로\대상\예제파일.zip -DestinationPath 경로\대상\추출된_파일` : ZIP 파일을 지정한 폴더에 압축 해제
- `Expand-Archive -Path 경로\대상\예제파일.zip -DestinationPath 경로\대상\추출된_파일 -Force` : 기존 파일을 덮어쓰며 압축 해제
- `Expand-Archive -Path 경로\대상\예제파일.zip -DestinationPath 경로\대상\추출된_파일 -WhatIf` : 실제 압축 해제하지 않고 수행 예정 작업 미리 보기
