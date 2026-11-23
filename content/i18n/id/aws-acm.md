# aws acm

AWS Certificate Manager, manajer sertifikat digital untuk AWS.

## Usage
- `aws acm import-certificate --certificate-arn arn_sertifikat --certificate sertifikat --private-key kunci_privat --certificate-chain rantai_sertifikat` : impor suatu sertifikat
- `aws acm list-certificates` : tampilkan daftar sertifikat
- `aws acm describe-certificate --certificate-arn arn_sertifikat` : tampilkan deskripsi suatu sertifikat
- `aws acm request-certificate --domain-name nama_domain --validation-method metode_validasi` : minta sertifikat baru bagi suatu domain
- `aws acm delete-certificate --certificate-arn arn_sertifikat` : hapus suatu sertifikat
- `aws acm list-certificates --certificate-statuses status` : tampilkan daftar status pengajuan validasi sertifikat
- `aws acm get-certificate --certificate-arn arn_sertifikat` : dapatkan informasi rincian suatu sertifikat
- `aws acm update-certificate-options --certificate-arn arn_sertifikat --options pengaturan` : mutakhirkan pengaturan terhadap suatu sertifikat

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/acm/
