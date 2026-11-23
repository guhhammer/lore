# compress-archive

PowerShell에서 압축(zipped) 아카이브 파일을 생성하는 cmdlet.

## Usage
- `Compress-Archive -Path 경로\대상\파일.txt -DestinationPath 경로\대상\파일.zip` : 단일 파일 압축
- `Compress-Archive -Path 경로\대상\파일1, 경로\대상\파일2, ... -DestinationPath 경로\대상\파일s.zip` : 여러 파일 압축
- `Compress-Archive -Path 경로\대상\디렉터리 -DestinationPath 경로\대상\디렉터리.zip` : 디렉터리 압축
- `Compress-Archive -Path 경로\대상\파일 -DestinationPath 경로\대상\디렉터리.zip -Update` : 기존 압축 파일 업데이트
- `Compress-Archive -Path 경로\대상\디렉터리 -DestinationPath 경로\대상\디렉터리.zip -CompressionLevel Optimal|Fastest|NoCompression` : 압축 수준 설정 (`Optimal`: 최대 압축, `Fastest`: 빠른 압축, `NoCompression`: 압축 안 함)
