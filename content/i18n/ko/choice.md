# choice

사용자에게 선택지를 제시하고 선택한 선택지의 색인을 반환합니다.

## Usage
- `choice` : 현재 사용자에게 `Y` 또는 `N` 선택지를 제시
- `choice /c AB` : 현재 사용자에게 특정 세트에서 choice 선택지를 제시
- `choice /m "메시지"` : 현재 사용자에게 특정 message와 함께 선택지를 제시
- `choice /cs /c Ab` : 현재 사용자에게 대소문자를 구분하는 case-sensitive choice 선택지를 특정 세트에서 제시
- `choice /t 5 /d N` : 현재 사용자에게 선택지를 제시하고, 특정 time 내에 default 선택지를 선호
- `choice /?` : 도움말 표시
