# ffmpeg

Video conversion tool.

## Usage
- `ffmpeg -i path/to/video.mp4 -vn path/to/sound.mp3` : extract the sound from a video and save it as MP3
- `ffmpeg -i path/to/input_audio.flac -ar 44100 -sample_fmt s16 path/to/output_audio.wav` : transcode a FLAC file to Red Book CD format (44100Hz, 16bit)
- `ffmpeg -i path/to/video.mp4 -vf 'scale=-1:1000' -r 15 path/to/output.gif` : save a video as GIF, scaling the height to 1000px and setting framerate to 15
- `ffmpeg -f image2 -i path/to/frame_%d.jpg video.mpg|video.gif` : combine numbered images (`frame_1.jpg`, `frame_2.jpg`, etc.) into a video or GIF
- `ffmpeg -i path/to/input_video.mp4 -ss mm:ss -to mm2:ss2 -c copy path/to/output_video.mp4` : trim a video from a given start time mm:ss to an end time mm2:ss2 (omit the -to flag to trim till the end)
- `ffmpeg -i path/to/input_video.avi -c:a aac -b:a 128k -c:v libx264 -crf 23 path/to/output_video.mp4` : convert AVI video to MP4. AAC Audio @ 128kbit, h264 Video @ CRF 23
- `ffmpeg -i path/to/input_video.mkv -c copy path/to/output_video.mp4` : remux MKV video to MP4 without re-encoding audio or video streams
- `ffmpeg -i path/to/input_video.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus -vbr on -threads number_of_threads path/to/output_video.webm` : convert MP4 video to VP9 codec. For the best quality, use a CRF value (recommended range 15-35) and -b:v MUST be 0

## See also
- gst-launch-1.0

## Categories
media

## Links
- More information: https://ffmpeg.org/ffmpeg.html#Options
- tldr page: https://tldr.inbrowser.app/pages/common/ffmpeg
