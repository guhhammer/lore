# rails generate

Generate new Rails templates in an existing project.

## Usage
- `rails generate` : list all available generators
- `rails generate model Post title:string body:text` : generate a new model named Post with attributes title and body
- `rails generate controller Posts index show new create` : generate a new controller named Posts with actions index, show, new, and create
- `rails generate migration AddCategoryToPost category:string` : generate a new migration that adds a category attribute to an existing model called Post
- `rails generate scaffold Post title:string body:text` : generate a scaffold for a model named Post, predefining the attributes title and body

## Categories
text, development

## Links
- More information: https://guides.rubyonrails.org/command_line.html#bin-rails-generate
- tldr page: https://tldr.inbrowser.app/pages/common/rails-generate
