# sattach

Slurm 작업 단계에 연결.

## Usage
- `sattach 작업_id.단계_id` : slurm 작업 단계의 IO 스트림(`stdout`, `stderr`, `stdin`)을 현재 터미널로 리디렉션
- `sattach --input-filter 작업_번호` : 현재 콘솔의 입력을 지정된 작업의 `stdin`으로 사용
- `sattach --출력|오류-filter 작업_번호` : 지정된 작업의 `stdin`/`stderr`만 리디렉션

## Links
- More information: https://slurm.schedmd.com/sattach.html
