# adb install

Android Debug Bridge Install: Instalar apps em uma instância do Android emulator ou dispositivos conectados.

## Usage
- `adb install caminho/para/arquivo.apk` : instala um app Android em um emulador/dispositivo
- `adb -s numero_de_serie install caminho/para/arquivo.apk` : instala um app Android em um emulador/dispositivo específico (sobrepõe `$ANDROID_SERIAL`)
- `adb install -r caminho/para/arquivo.apk` : reinstala um app existente, mantendo seus dados
- `adb install -d caminho/para/arquivo.apk` : instala um app Android permitindo um downgrade de versão de código (apenas pacotes depuráveis)
- `adb install -g caminho/para/arquivo.apk` : concede todas as permissões listadas no manifesto do app
- `adb install --fastdeploy caminho/para/arquivo.apk` : atualiza rapidamente um app já instalado atualizando apenas as partes do APK que mudaram

## Links
- More information: https://developer.android.com/tools/adb
