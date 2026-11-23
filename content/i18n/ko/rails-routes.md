# rails routes

Rails 애플리케이션에서 경로를 나열.

## Usage
- `rails routes` : 모든 경로 나열
- `rails routes -E` : 확장된 형식으로 모든 경로 나열
- `rails routes -g posts_path|GET|/posts` : URL 헬퍼 메서드 이름, HTTP 메서드 또는 URL 경로와 부분적으로 일치하는 경로 나열
- `rails routes -c posts|Posts|Blogs::PostsController` : 지정된 컨트롤러에 매핑된 경로 나열

## Links
- More information: https://guides.rubyonrails.org/routing.html
