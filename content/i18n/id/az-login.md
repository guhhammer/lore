# az login

Masuk ke dalam platform Azure. Bagian dari `azure-cli` (juga dikenal sebagai `az`).

## Usage
- `az login` : lakukan proses masuk/login secara interaktif
- `az login --service-principal -u http://perwakilan-layanan-azure-cli -p rahasia -t seseorang.onmicrosoft.com` : masuk dengan suatu perwakilan layanan menggunakan suatu token rahasia klien
- `az login --service-principal -u http://perwakilan-layanan-azure-cli -p jalan/menuju/sertifikat.pem -t seseorang.onmicrosoft.com` : masuk dengan suatu perwakilan layanan menggunakan suatu sertifikat klien
- `az login -i` : masuk menggunakan identitas bawaan suatu sistem VM
- `az login -i -u /subscriptions/id_langganan/resourcegroups/grup_sumber_daya_saya/providers/Microsoft.ManagedIdentity/userAssignedIdentities/id_saya` : masuk menggunakan identitas pengguna yang dipadankan ke dalam instansi VM

## Links
- More information: https://learn.microsoft.com/cli/azure/reference-index#az-login
