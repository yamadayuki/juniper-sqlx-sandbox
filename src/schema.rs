use crate::context::Context;
use juniper::{EmptyMutation, EmptySubscription, GraphQLEnum, GraphQLObject, RootNode};

#[derive(GraphQLEnum, Debug, Clone, Copy, sqlx::Type)]
#[graphql(description = "Represents a user role")]
#[sqlx(type_name = "role", rename_all = "lowercase")]
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

#[derive(GraphQLObject, Debug, Clone)]
struct Post {
    id: i32,
    title: String,
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn actor(
        context: &Context,
        #[graphql(desc = "id of the actor")] id: i32,
    ) -> Option<Actor> {
        let actor = sqlx::query_as!(
            Actor,
            r#"SELECT id, name, role as "role!: Role" FROM actors WHERE id = $1"#,
            id
        )
        .fetch_optional(&context.pool)
        .await
        .unwrap();

        dbg!(&actor);

        actor
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
}
