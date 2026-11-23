# mkdir

Dizinleri oluştur ve izinlerini ayarla.

## Usage
- `mkdir yol/dizin1 yol/dizin2 ...` : belirtilen dizinleri oluştur
- `mkdir -p yol/dizin1 yol/dizin2 ...` : belirtilen dizinleri ve gerekiyorsa üst dizinlerini oluştur
- `mkdir -m u=rwx,g=rw,o=r yol/dizin1 yol/dizin2 ...` : belirtilen izinlerle dizinleri oluştur
- `mkdir -p yol/{a,b}/{x,y,z}/{h,i,j}` : birden fazla iç içe dizini özyinelemeli olarak oluştur
