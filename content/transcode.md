# transcode

Transcode video and audio codecs, and convert between media formats.

## Usage
- `transcode -J stabilize -i input_file` : create stabilization file to be able to remove camera shakes
- `transcode -J transform -i input_file -y xvid -o output_file` : remove camera shakes after creating stabilization file, transform video using XviD
- `transcode -Z 640x480 -i input_file -y xvid -o output_file` : resize the video to 640x480 pixels and convert to MPEG4 codec using XviD

## Categories
media

## Links
- More information: https://manned.org/transcode
- tldr page: https://tldr.inbrowser.app/pages/common/transcode
