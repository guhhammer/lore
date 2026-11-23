# docker container run

Yeni bir Docker konteynerinde bir komut çalıştır.

## Usage
- `docker run imge:etiket komut` : yeni bir konteynerde, etiketlenmiş bir imgeden komut çalıştır
- `docker run -d imge komut` : yeni bir konteynerde arkaplanda çalışacak şekilde komut çalıştır ve ID'sini göster
- `docker run --rm -it imge komut` : i̇nteraktif mod ve pseudo-TTY'deki bir açık-kapalı konteynerde komut çalıştır
- `docker run -e 'değişken=değer' -e değişken imge komut` : yeni bir konteynerde geçebilmiş çevresel değişkenler ile komut çalıştır
- `docker run -v /örnek/host:/örnek/konteyner imge komut` : yeni bir konteynerde bağlama takılı hacimlerle komut çalıştır
- `docker run -p host_portu:konteyner_portu imge komut` : yayınlanmış portları içeren yeni bir konteynerde komut çalıştır

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
