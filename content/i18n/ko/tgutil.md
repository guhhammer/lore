# tgutil

Telegram 계정 관리.

## Usage
- `tgutil -n 세션_이름 -u 채팅_아이디 -m edit --text "현재_텍스트" --newtext "새로운_텍스트"` : 채팅에서 지정한 텍스트와 일치하는 마지막 메시지를 새로운 텍스트로 수정
- `tgutil -n 세션_이름 -u 채팅_아이디 -m editall --text "현재_텍스트" --newtext "새로운_텍스트"` : `현재_텍스트`를 포함하는 모든 메시지를 `새로운_텍스트`로 수정
- `tgutil -n 세션_이름 -u 채팅_아이디 -m delete --text "현재_텍스트"` : 채팅에서 지정한 텍스트와 일치하는 마지막 메시지를 삭제
- `tgutil -n 세션_이름 -u 채팅_아이디 -m deleteall --text "검색어"` : 지정한 `검색어`를 포함하는 모든 메시지 삭제

## Links
- More information: https://pypi.org/project/telegram-cloud/
