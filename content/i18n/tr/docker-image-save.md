# docker image save

Bir veya daha fazla Docker imgesini arşivlemek için dışa aktar.

## Usage
- `docker save imge:etiket >örnek/dosya.tar` : bir imgeyi, `stdout`'u `.tar` arşivine yönlendirerek kaydet
- `docker save -o örnek/dosya.tar imge:etiket` : bir imgeyi, bir `.tar` arşivine kaydet
- `docker save -o örnek/dosya.tar imge_ismi` : bir imgenin tüm etiketlerini kaydet
- `docker save -o örnek/dosya.tar imge_ismi:etiket1 imge_ismi:etiket2 ...` : bir imgenin belirli etiketlerini kaydetmek için elle seç

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
