# java

Средство запуска Java-приложений.

## Usage
- `java имя_класса` : выполнить файл Java `.class`, содержащий метод main, используя только имя класса
- `java -classpath путь/к/классам1:путь/к/классам2:. имя_класса` : выполнить Java-программу с использованием дополнительных сторонних или пользовательских классов
- `java -jar имя_файла.jar` : выполнить `.jar`-программу
- `java -agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=*:5005 -jar имя_файла.jar` : выполнить `.jar`-программу с отладкой, ожидающей подключения на порту 5005
- `java -help` : показать справку
- `java -version` : показать версии JDK, JRE и HotSpot

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/java.html
