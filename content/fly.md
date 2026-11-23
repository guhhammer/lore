# fly

Tool for concourse-ci.

## Usage
- `fly -t target_name login -n team_name -c https://ci.example.com` : authenticate with and save concourse target
- `fly targets` : list targets
- `fly -t target_name pipelines` : list pipelines
- `fly -t target_name set-pipeline -c pipeline.yml -p pipeline_name` : upload or update a pipeline
- `fly -t target_name unpause-pipeline -p pipeline_name` : unpause pipeline
- `fly -t target_name get-pipeline -p pipeline_name` : show pipeline configuration
- `fly -t target_name sync` : update local copy of fly
- `fly -t target_name destroy-pipeline -p pipeline_name` : destroy pipeline

## Categories
network

## Links
- More information: https://concourse-ci.org/fly.html
- tldr page: https://tldr.inbrowser.app/pages/common/fly
