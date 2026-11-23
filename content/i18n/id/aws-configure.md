# aws configure

Atur konfigurasi penggunaan AWS CLI.

## Usage
- `aws configure` : atur konfigurasi AWS CLI secara interaktif (akan membuat konfigurasi baru atau memutakhirkan konfigurasi bawaan)
- `aws configure --profile nama_profil` : atur konfigurasi bagi suatu profil pengguna AWS CLI (akan membuat profil pengguna baru atau memutakhirkannya)
- `aws configure get nama_variabel` : tampilkan nilai terhadap suatu variabel konfigurasi
- `aws configure get nama_variabel --profile nama_profil` : tampilkan nilai terhadap suatu variabel konfigurasi pada suatu profil pengguna
- `aws configure set nama_variabel nilai` : atur nilai suatu variabel konfigurasi
- `aws configure set nama_variabel nilai --profile nama_profil` : atur nilai suatu variabel konfigurasi bagi suatu profil pengguna
- `aws configure list` : tampilkan seluruh konfigurasi yang disimpan
- `aws configure list --profile nama_profil` : tampilkan seluruh konfigurasi yang disimpan bagi suatu profil pengguna

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/configure/
