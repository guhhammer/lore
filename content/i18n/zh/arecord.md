# arecord

ALSA 声卡驱动的声音录制器。

## Usage
- `arecord -vv -f cd 路径/文件名.wav` : 以 "CD" 质量录制一段声音（录制结束以 `<Ctrl c>` 停止）
- `arecord -vv -f cd -d 10 路径/文件名.wav` : 以 "CD" 质量录制 10 秒钟声音
- `arecord -vv -f cd -t raw | lame -r - 路径/文件名.mp3` : 录制一段声音并以 mp3 格式保存（录制结束以 `<Ctrl c>` 停止）
- `arecord -l` : 列出所有的声卡和数字音频设备
- `arecord -i` : 允许交互式界面（例如使用`<Space>`或`<Enter>`播放或暂停）
- `arecord -d 5 test-mic.wav && aplay test-mic.wav && rm test-mic.wav` : 通过录制 5 秒音频样本并回放来测试麦克风

## Links
- More information: https://manned.org/arecord
