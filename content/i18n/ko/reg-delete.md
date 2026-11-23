# reg delete

레지스트리에서 키 또는 해당 값을 삭제.

## Usage
- `reg delete 키_이름` : 특정 레지스트리 키 삭제
- `reg delete 키_이름 /v 값` : 특정 키 아래의 value 삭제
- `reg delete 키_이름 /va` : 지정된 키 아래의 모든 value를 재귀적으로 삭제
- `reg delete 키_이름 /f /va` : 프롬프트 없이 강제로 특정 키 아래의 모든 value를 재귀적으로 삭제
