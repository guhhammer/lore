# bdfr

Reddit용 대량 다운로더.

## Usage
- `bdfr download 경로/대상/출력_디렉터리 -l 게시_url` : 지정된 링크(links)에서 게시물의 URL 또는 ID로 비디오/이미지를 다운로드
- `bdfr download 경로/대상/출력_디렉터리 -u reddit_사용자 --submitted` : 지정된 사용자(user)로부터 가능한 최대 수(약 1000개)의 비디오/이미지를 다운로드
- `bdfr archive 경로/대상/출력_디렉터리 -s 'Python, all, mindustry' -L 10` : 제출 데이터 (텍스트, 공감, 댓글 등) 다운로드를 각 subreddit에 대해 10개의 제출로 제한됨 (Limited) (총 30개)
- `bdfr download 경로/대상/출력_디렉터리 -s Python -S top -t all -L 10` : subreddit r/Python에서 시간(time) 필터 전체를 사용하여 상위 순으로 정렬됨(Sorted) (기본값은 hot), 제출은 10개로 제한됨(Limited)
- `bdfr clone 경로/대상/출력_디렉터리 -s Python --skip mp4 --skip gif --make-hard-links` : subreddit r/Python에서 제출 데이터와 비디오/이미지 모두 가능한 최대 수를 다운로드하고, mp4 또는 gif 파일 확장자를 가진 제출을 건너뛰고 중복 파일에 대한 하드 링크를 생성
- `bdfr download 경로/대상/출력_디렉터리 -u me --saved --authenticate --file-scheme ' {POSTID}_{TITLE}_{UPVOTES} ' --no-dupes --search-existing` : 지정된 형식에 따라 각 파일의 이름을 지정하여, 인증된 사용자의 저장된 게시물을 다운로드. 출력 디렉터리에 이미 존재하는 중복 항목과 게시물을 다운로드하지 않음

## Links
- More information: https://github.com/Serene-Arc/bulk-downloader-for-reddit
