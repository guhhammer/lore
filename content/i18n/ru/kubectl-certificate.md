# kubectl certificate

Управлять запросами на подпись сертификатов.

## Usage
- `kubectl certificate approve имя_csr` : одобрить запрос на подпись сертификата по имени
- `kubectl certificate deny имя_csr` : отклонить запрос на подпись сертификата по имени
- `kubectl certificate approve --filename путь/к/csr.yaml` : одобрить запрос на подпись сертификата, определённый в файле манифеста
- `kubectl certificate deny --filename путь/к/csr.yaml` : отклонить запрос на подпись сертификата, определённый в файле манифеста
- `kubectl certificate approve --force имя_csr` : принудительно переодобрить запрос на подпись сертификата, который был ранее отклонён

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_certificate/
