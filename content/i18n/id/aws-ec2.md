# aws ec2

Atur daftar instansi dan volume penyimpanan AWS EC2. AWS EC2 menyediakan kapasitas komputasi yang aman dan dapat diubah ukurannya dalam platform komputasi awan AWS untuk pengembangan dan penerapan aplikasi yang lebih cepat.

## Usage
- `aws ec2 describe-instances --instance-ids id_instansi` : tampilkan informasi mengenai suatu instansi EC2
- `aws ec2 describe-instances` : tampilkan informasi mengenai seluruh instansi EC2 terdaftar
- `aws ec2 describe-volumes` : tampilkan informasi mengenai seluruh volume penyimpanan EC2 terdaftar
- `aws ec2 delete-volume --volume-id id_volume` : hapus suatu volume penyimpanan EC2
- `aws ec2 create-snapshot --volume-id id_volume` : buat suatu tangkapan isi (snapshot) suatu volume penyimpanan EC2
- `aws ec2 describe-images` : tampilkan daftar cakram AMI (Amazon Machine Image) yang tersedia
- `aws ec2 help` : tampilkan seluruh perintah EC2 yang tersedia
- `aws ec2 subperintah help` : tampilkan bantuan spesifik untuk suatu subperintah EC2

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ec2/
