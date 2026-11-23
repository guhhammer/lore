# ffprobe

मल्टीमीडिया स्ट्रीम विश्लेषक।

## Usage
- `ffprobe -v error -show_streams इनपुट.mp4` : मीडिया फ़ाइल की स्ट्रीम से जुड़ी सभी उपलब्ध जानकारी दिखाएं
- `ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 इनपुट.mp4` : मीडिया की अवधि दिखाएं
- `ffprobe -v error -select_streams v:0 -show_entries stream=avg_frame_rate -of default=noprint_wrappers=1:nokey=1 इनपुट.mp4` : वीडियो की फ़्रेम दर दिखाएं
- `ffprobe -v error -select_streams v:0 -show_entries stream=width|height -of default=noprint_wrappers=1:nokey=1 इनपुट.mp4` : वीडियो की चौड़ाई या ऊंचाई दिखाएं
- `ffprobe -v error -select_streams v:0 -show_entries stream=bit_rate -of default=noprint_wrappers=1:nokey=1 इनपुट.mp4` : वीडियो की औसत बिट दर दिखाएं

## Links
- More information: https://ffmpeg.org/ffprobe.html
