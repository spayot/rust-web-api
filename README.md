# simple Web API Example
derived from the excellent project walk through series from tensor: 
* specific episode: https://youtu.be/c6q0lUtD3FY?si=Xav0vaxeTmtlnJQf


## Launch API service locally
`cargo run`

## Query API
* retrieve all existing posts 
`curl -X GET "localhost:9200/post_feed" | jq '.'`
