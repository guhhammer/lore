# out-string

입력 객체를 문자열로 출력합니다. 참고: 이 명령어는 PowerShell을 통해서만 사용할 수 있습니다.

## Usage
- `Get-Alias | Out-String` : 호스트 정보를 문자열로 출력
- `Get-Alias | Out-String -Stream` : 모든 객체를 단일 문자열로 연결하는 대신 각 객체를 문자열로 변환
- `@{TestKey = ('x' * 200)} | Out-String -Width 250` : `Width` 매개변수를 사용하여 잘림을 방지
