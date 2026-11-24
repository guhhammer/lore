# piper

A fast, local neural text to speech system. Try out and download speech models from <https://rhasspy.github.io/piper-samples>.

## Usage
- `echo Thing to say | piper -m path/to/model.onnx -f outputfile.wav` : output a WAV file using a text-to-speech model (assuming a configuration file at model_path + `.json`)
- `echo Thing to say | piper -m path/to/model.onnx -c path/to/model.onnx.json -f outputfile.wav` : output a WAV file using a model and specifying its JSON config file
- `echo Warum? | piper -m de_DE-thorsten_emotional-medium.onnx --speaker 1 -f angry.wav` : select a particular speaker in a voice with multiple speakers by specifying the speaker's ID number
- `echo Hello world | piper -m en_GB-northern_english_male-medium.onnx --output-raw -f - | mpv -` : stream the output to the mpv media player
- `echo Speaking twice the speed. With added drama! | piper -m file.onnx --length_scale 0.5 --sentence_silence 2 -f drama.wav` : speak twice as fast, with huge gaps between sentences

## Categories
files, network, text

## Links
- More information: https://github.com/OHF-Voice/piper1-gpl
- tldr page: https://tldr.inbrowser.app/pages/common/piper
- Wikipedia: https://en.wikipedia.org/wiki/Piper_(source_control_system)
