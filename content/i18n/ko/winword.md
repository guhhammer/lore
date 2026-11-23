# winword

Microsoft Office 워드 프로세스 애플리케이션.

## Usage
- `winword` : microsoft Word 실행
- `winword /n` : 문서를 열지 않고(no) Microsoft Word 실행
- `winword /w` : 새 빈 문서 작성(write)
- `winword /f 경로\대상\파일.docx` : 기존 파일을 기반으로(from) 새 문서 생성
- `winword /t 경로\대상\파일1.docx 경로\대상\파일2.docx ...` : 하나 이상의 문서를 편집용(editing)으로 열기
- `winword /m 경로\대상\파일.docm` : 모든 AutoExec 매크로(macros)를 비활성화한 상태로 문서 열기
- `winword /m매크로_이름 경로\대상\파일.docm` : 모든 AutoExec 매크로(macros)를 비활성화한 상태로 문서를 연 후, 지정한 매크로 실행
- `winword /safe` : 안전 모드(Safe Mode)로 Microsoft Word 실행
