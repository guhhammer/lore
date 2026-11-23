# input

Stuur gebeurteniscodes of touchscreen-gebaren naar een Android-apparaat. Dit commando kan alleen worden gebruikt via `adb shell`.

## Usage
- `input keyevent event_code` : stuur een gebeurteniscode voor een enkel teken naar een Android-apparaat
- `input text "text"` : stuur een tekst naar een Android-apparaat (`%s` vertegenwoordigt spaties)
- `input tap x_position y_position` : stuur een enkele tik naar een Android-apparaat
- `input swipe x_start y_start x_end y_end duration_in_ms` : stuur een swipe-gebaar naar een Android-apparaat
- `input swipe x_position y_position x_position y_position duration_in_ms` : stuur een lange druk naar een Android-apparaat met behulp van een swipe-gebaar
