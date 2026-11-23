# dotnet run

명시적인 컴파일 또는 실행 명령 없이 .NET 애플리케이션을 실행합니다.

## Usage
- `dotnet run` : 현재 디렉토리의 프로젝트 실행
- `dotnet run --project 경로/대상/파일.csproj` : 특정 프로젝트 실행
- `dotnet run -- arg1=foo arg2=bar ...` : 특정 인수를 사용하여 프로젝트 실행
- `dotnet run -f net7.0` : 대상 프레임워크 모니커를 사용하여 프로젝트 실행
- `dotnet run -a x86|x64|arm|arm64 --os win|win7|osx|linux|ios|android` : .NET 6부터 사용 가능한 아키텍처 및 OS 지정 (이 옵션들과 함께 `--runtime` 사용 금지)

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-run
