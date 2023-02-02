use juniper::{EmptyMutation, EmptySubscription, GraphQLEnum, GraphQLObject, RootNode};

#[derive(GraphQLEnum, Debug, Clone, Copy)]
#[graphql(description = "Represents a user role")]
enum Role {
    Admin,
    Editor,
    Viewer,
}

#[derive(Debug)]
struct Actor {
    id: i32,
    name: String,
    role: Role,
    posts: Vec<Post>,
}

#[juniper::graphql_object]
impl Actor {
    fn id(&self) -> i32 {
        self.id * 10
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn role(&self) -> Role {
        self.role
    }

    fn posts(&self) -> Vec<Post> {
        self.posts.clone()
    }
}

#[derive(GraphQLObject, Debug, Clone)]
struct Post {
    id: i32,
    title: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn actor() -> Actor {
        Actor {
            id: 1,
            name: "John Doe".to_string(),
            role: Role::Admin,
            posts: vec![
                Post {
                    id: 1,
                    title: "Hello World".to_string(),
                },
                Post {
                    id: 2,
                    title: "New Era".to_string(),
                },
            ],
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
}
