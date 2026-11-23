# mmls

볼륨 시스템의 파티션 레이아웃을 표시.

## Usage
- `mmls 경로/대상/이미지_파일` : 이미지 파일에 저장된 파티션 테이블 표시
- `mmls -B -i 경로/대상/이미지_파일` : 파티션 크기를 포함한 추가 열과 함께 파티션 테이블 표시
- `mmls -i ewf 이미지.e01 이미지.e02` : 분할된 EWF 이미지에서 파티션 테이블 표시
- `mmls -t 중첩_테이블_유형 -o 오프셋 경로/대상/이미지_파일` : 중첩된 파티션 테이블 표시

## Links
- More information: https://wiki.sleuthkit.org/Mmls/
