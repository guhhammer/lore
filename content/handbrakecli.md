# handbrakecli

Command-line interface to the HandBrake video conversion and DVD ripping tool.

## Usage
- `handbrakecli -i input.avi -o output.mkv -e x264 -q 20 -B 160` : convert a video file to MKV (AAC 160kbit audio and x264 CRF20 video)
- `handbrakecli -i input.mp4 -o output.mp4 -w 320 -l 240` : resize a video file to 320x240
- `handbrakecli -z` : list available presets
- `handbrakecli -Z "Android" -i input.ext -o output.mp4` : convert an AVI video to MP4 using the Android preset
- `handbrakecli -i /dev/sr0 -t 0` : print the content of a DVD, getting the CSS keys in the process
- `handbrakecli -i /dev/sr0 -t 1 -o out.mkv -f av_mkv -e x264 -s 1,4,5 -a 1,2 -E copy -q 23` : rip the first track of a DVD in the specified device. Audiotracks and subtitle languages are specified as lists

## Categories
media, network, text

## Links
- More information: https://handbrake.fr/docs/en/latest/cli/command-line-reference.html
- tldr page: https://tldr.inbrowser.app/pages/common/handbrakecli
