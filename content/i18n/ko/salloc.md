# salloc

클러스터에서 하나 이상의 노드를 할당하여 대화형 셸 세션을 시작하거나 명령을 실행합니다.

## Usage
- `salloc` : 클러스터의 노드에서 대화형 셸 세션 시작
- `salloc ls -a` : 클러스터의 노드에서 지정된 명령을 동기적으로 실행
- `salloc -C (amd|intel)&gpu` : 지정된 제약 조건을 충족하는 노드만 할당

## Links
- More information: https://slurm.schedmd.com/salloc.html
