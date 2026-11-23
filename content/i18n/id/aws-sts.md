# aws sts

Security Token Service (STS), layanan manajemen token keamanan untuk meminta akses akun sementara bagi pengguna (IAM) atau pengguna dari federasi.

## Usage
- `aws sts assume-role --role-arn aws_role_arn` : dapatkan kredensial keamanan sementara untuk mengakses sumber daya AWS tertentu
- `aws sts get-caller-identity` : dapatkan nama pengguna IAM atau peran (role) pengguna yang terikat kepada kredensial untuk melakukan operasi pengaturan layanan AWS

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sts/
