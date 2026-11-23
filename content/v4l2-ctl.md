# v4l2-ctl

Control video devices.

## Usage
- `v4l2-ctl -A` : list all video devices
- `v4l2-ctl --list-formats-ext` : list supported video formats and resolutions of default video device `/dev/video0`
- `v4l2-ctl --list-formats-ext -d path/to/video_device` : list supported video formats and resolutions of a specific video device
- `v4l2-ctl --all -d path/to/video_device` : get all details of a video device
- `v4l2-ctl -d path/to/video_device --set-fmt-video=width=width,height=height,pixelformat=MJPG --stream-mmap --stream-to=path/to/output.jpg --stream-count=1` : capture a JPEG photo with a specific resolution from video device
- `v4l2-ctl -d path/to/video_device --set-fmt-video=width=width,height=height,pixelformat=format --stream-mmap --stream-to=path/to/output --stream-count=number_of_frames_to_capture` : capture a raw video stream from video device
- `v4l2-ctl -l -d path/to/video_device` : list all video device's controls and their values
- `v4l2-ctl -d path/to/video_device -c control_name=value` : set the value of a video device control

## See also
- cam

## Categories
media, hardware, files

## Links
- More information: https://manned.org/v4l2-ctl
- tldr page: https://tldr.inbrowser.app/pages/linux/v4l2-ctl
