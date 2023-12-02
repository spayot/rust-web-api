# simple Web API Example
derived from the excellent project walk-through series from Tensor: 
* specific episode: https://youtu.be/c6q0lUtD3FY?si=Xav0vaxeTmtlnJQf
* github repo: https://github.com/tensor-programming/Rust_web_api/tree/master

API allows to access a (fake) database of blogposts.

## Launch API service locally
`cargo run`

## Query API
* retrieve all existing posts 
```bash
curl -X GET "localhost:9200/post_feed" | jq '.'
```

* add a new post:
```bash
curl -X POST -H "Content-Type: application/json" localhost:9200/post_post -d @- <<EOF
{
    "title": "Why not a 4th Title",
    "author": "Mr Body",
    "body": "good content, right there!",
    "datetime": "2023-12-02T07:30:33.675601Z",
    "uuid": "c2b86910-3eb0-40c1-88a9-b6fd7e1d71c8"
  } 
EOF
```

