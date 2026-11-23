# piper

Un sistema neural rápido y local de conversión de texto a voz.

## Usage
- `echo Cosa a decir | piper -m ruta/a/modelo.onnx -f archivo_de_salida.wav` : genera un archivo WAV utilizando un modelo de texto a voz (suponiendo un archivo de configuración en model_path + .json)
- `echo 'Lo que hay que decir' | piper -m ruta/a/modelo.onnx -c ruta/a/modelo.onnx.json -f archivo_de_salida.wav` : genera un archivo WAV utilizando un modelo y especificando su archivo de configuración JSON
- `echo 'Warum?' | piper -m de_DE-thorsten_emotional-medium.onnx --speaker 1 -f enojado.wav` : selecciona un locutor concreto en una voz con varios locutores especificando el número de identificación del locutor
- `echo 'Hello world' | piper -m en_GB-northern_english_male-medium.onnx --output-raw -f - | mpv -` : transmite la salida al reproductor multimedia mpv
- `echo 'Hablando el doble de rápido. Con más drama!' | piper -m foo.onnx --length_scale 0.5 --sentence_silence 2 -f drama.wav` : habla el doble de rápido, con grandes espacios entre frases

## Links
- More information: https://rhasspy.github.io/piper-samples
