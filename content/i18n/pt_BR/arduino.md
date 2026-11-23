# arduino

Arduino Studio - Ambiente de Desenvolvimento Integrado para a plataforma Arduino.

## Usage
- `arduino --verify caminho/para/arquivo.ino` : compila um sketch
- `arduino --upload caminho/para/arquivo.ino` : compila e envia sketch
- `arduino --board arduino:avr:nano:cpu=atmega328p --port /dev/ttyACM0 --upload caminho/para/arquivo.ino` : compila e envia sketch para um Arduino Nano com uma CPU Atmega328p, conectada na porta `/dev/ttyACM0`
- `arduino --pref nome=valor` : define a preferência `nome` para um determinado `valor`
- `arduino --pref build.path=caminho/para/diretório --verify caminho/para/arquivo.ino` : compila um sketch, coloca o resultado da compilação no diretório de compilação, e reutiliza qualquer resultado pre-existente neste diretório
- `arduino --save-prefs` : salva todas as preferências (alteradas) para `preferences.txt`
- `arduino --install-boards "arduino:sam"` : instala a última placa SAM
- `arduino --install-library "Bridge:1.0.0,Servo:1.2.0"` : instala bibliotecas Bridge e Servo

## Links
- More information: https://github.com/arduino/Arduino/blob/master/build/shared/manpage.adoc
