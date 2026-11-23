# spa-resample

Resample an audio file using the PipeWire resampler. Note: This utility is primarily intended for testing and debugging the resampler.

## Usage
- `spa-resample path/to/input.wav path/to/output.wav` : resample an audio file
- `spa-resample -r 48000 path/to/input.wav path/to/output.wav` : resample an audio file to a specific sample rate
- `spa-resample -f s8|s16|s32|f32|f64 path/to/input.wav path/to/output.wav` : resample an audio file to a specific format
- `spa-resample -q 0..14 path/to/input.wav path/to/output.wav` : resample an audio file with a specific quality (0 is lowest, 14 is highest)
- `spa-resample -h` : display help

## Categories
files, media

## Links
- More information: https://docs.pipewire.org/page_man_spa-resample_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/spa-resample
