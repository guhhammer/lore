# lib

Microsoft Library Manager(라이브러리 관리 도구)로, 오브젝트 파일로부터 정적 라이브러리를 생성하고 관리.

## Usage
- `lib /OUT :경로\대상\라이브러리.lib 경로\대상\파일1.obj 경로\대상\파일2.obj ...` : 오브젝트 파일로부터 정적 라이브러리 생성
- `lib /LIST 경로\대상\라이브러리.lib` : 라이브러리의 내용 목록 표시
- `lib 경로\대상\라이브러리.lib 경로\대상\파일.obj` : 기존 라이브러리에 오브젝트 파일 추가
- `lib /REMOVE :경로\대상\파일.obj 경로\대상\라이브러리.lib` : 라이브러리에서 오브젝트 파일 제거
- `lib /EXTRACT :경로\대상\파일.obj 경로\대상\라이브러리.lib` : 라이브러리에서 오브젝트 파일 추출
- `lib /DEF :path\to\definition.def /OUT:path\to\import.lib` : DLL로부터 Import 라이브러리 생성

## Links
- More information: https://learn.microsoft.com/cpp/build/reference/lib-reference
