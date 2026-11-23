# gcloud

Офіційний CLI інструмент для Google Cloud Platform. Примітка: підкоманди `gcloud` мають власну документацію щодо використання.

## Usage
- `gcloud config list` : вивести всі властивості в активній конфігурації
- `gcloud auth login` : увійти в обліковий запис Google
- `gcloud config set project назва_проекту` : встановити активний проект
- `gcloud compute ssh користувач@екземпляр` : SSH в екземпляр віртуальної машини
- `gcloud compute instances list` : вивести всі екземпляри Google Compute Engine у проекті (за замовчуванням виводяться екземпляри з усіх зон)
- `gcloud container clusters get-credentials назва_кластера` : оновити файл kubeconfig відповідними обліковими даними, щоб вказати `kubectl` на певний кластер у Google Kubernetes Engine (GKE)
- `gcloud components update` : оновити усі компоненти `gcloud`
- `gcloud help команда` : вивести довідки для заданої команди

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud
