# gcloud projects

Manage project access policies in Google Cloud.

## Usage
- `gcloud projects create project_id|project_number` : create a new project
- `gcloud projects list` : list all active projects
- `gcloud projects describe project_id` : display metadata for a project
- `gcloud projects delete project_id|project_number` : delete a project
- `gcloud projects add-iam-policy-binding project_id --member principal --role role` : add an IAM policy binding to a specified project

## Categories
cloud, development

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/projects
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-projects
