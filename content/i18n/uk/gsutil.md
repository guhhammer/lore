# gsutil

Доступ до Google Cloud Storage. Ви можете використовувати `gsutil` для виконання широкого діапазону завдань із керування контейнерами (buckets) та об’єктами.

## Usage
- `gsutil ls` : вивести всі контейнери (buckets) в проекті, до якого ви ввійшли
- `gsutil ls -r 'gs://bucket_name/prefix**'` : вивести об’єкти у контейнері (bucket)
- `gsutil cp gs://bucket_name/ім'я_об'єкта шлях/де/зберегти_розташування` : завантажити об'єкт із контейнера (bucket)
- `gsutil cp місцезнаходження_об'єкта gs://destination_bucket_name/` : завантажити об’єкт у контейнер (bucket)
- `gsutil mv gs://bucket_name/старе_ім'я_об'єкта gs://bucket_name/нове_ім'я_об'єкта` : перейменувати або перемістіти об’єкти у контейнері (bucket)
- `gsutil mb gs://bucket_name` : створити новий контейнер (bucket) в проекті, у який ви ввійшли
- `gsutil rm -r gs://bucket_name` : видалити контейнер (bucket) та видалити всі об’єкти в ньому

## Links
- More information: https://docs.cloud.google.com/storage/docs/gsutil
