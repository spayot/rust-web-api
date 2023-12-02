use crate::models::Post;

pub struct DataBase {
    posts: Vec<Post>,
}

impl DataBase {
    pub fn new() -> DataBase {
        DataBase { posts: vec![] }
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn posts(&self) -> &Vec<Post> {
        &self.posts
    }
}
