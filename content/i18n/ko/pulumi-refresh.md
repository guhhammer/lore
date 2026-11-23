# pulumi refresh

스택의 리소스 상태를 새로 고침.

## Usage
- `pulumi refresh` : 현재 스택의 상태를 cloud provider의 실제 상태와 비교하고 변경 사항을 현재 스택에 반영
- `pulumi refresh --diff` : 현재 스택의 리소스 상태를 새로 고치고 작업 내용을 상세한 Diff로 표시
- `pulumi refresh --expect-no-changes` : 현재 스택의 리소스 상태를 새로 고치고 새로고침동안 변경 사항이 발생하면 오류 반환
- `pulumi refresh --preview-only` : 실제로 새로 고치지 않고, 변경될 내용만 미리보기
- `pulumi refresh -s 스택_이름` : 작업할 스택을 지정 (기본값: 현재 스택)
- `pulumi refresh -h` : 도움말 표시

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_refresh/
