use super::post::Post;
use super::role::Role;
use crate::context::Context;

#[derive(Debug)]
pub(crate) struct Actor {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) role: Role,
    // posts: Vec<Post>,
}

#[juniper::graphql_object(Context = Context)]
impl Actor {
    fn id(&self, _context: &Context) -> i32 {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn role(&self) -> Role {
        self.role
    }

    fn posts(&self) -> Vec<Post> {
        // self.posts.clone()
        vec![]
    }
}
