# java

Peluncur Aplikasi Java.

## Usage
- `java nama_class` : jalankan berkas java `.class` yang mengandung method main dengan hanya menggunakan nama class
- `java -classpath jalan/menuju/direktori_class1:jalan/menuju/direktori_class2:. nama_class` : jalankan sebuah program java menggunakan berkas-berkas `.class` eksternal dan tambahan
- `java -jar nama_berkas.jar` : jalankan program `.jar`
- `java -agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=*:5005 -jar nama_berkas.jar` : jalankan program `.jar` dengan menunggu debugger terhubung ke port 5005
- `java -help` : tampilkan bantuan
- `java -version` : tampilkan versi JDK, JRE dan HotSpot

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/java.html
