# crane append

(선택 사항) 기본 이미지를 기반으로 이미지를 푸시. 제공된 tarball의 내용이 포함된 레이어를 추가.

## Usage
- `crane append -b 이미지_이름` : 기본 이미지를 기반으로 하는 이미지 푸시
- `crane append -f 레이어_이름1 레이어_이름2 ...` : tarball에서 추가된 레이어가 있는 이미지 푸시
- `crane append -t 태그_이름` : 새로운 태그가 포함된 레이어가 추가된 이미지 푸시
- `crane append -o 경로/대상/tarball` : 결과 이미지를 새 tarball로 푸시
- `crane append --oci-empty-base` : docker 대신 OCI 미디어 유형의 비어있는 기본 이미지를 사용
- `crane append --set-base-image-annotations` : 기본 이미지를 기반으로 결과 이미지에 주석을 달기
- `crane append -h` : 도움말 표시
