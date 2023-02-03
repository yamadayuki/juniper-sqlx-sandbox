use crate::context::Context;
use juniper::{EmptySubscription, GraphQLEnum, GraphQLObject, RootNode};

#[derive(GraphQLEnum, Debug, Clone, Copy, sqlx::Type)]
#[graphql(description = "Represents a user role")]
#[sqlx(type_name = "actor_role", rename_all = "lowercase")]
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

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(name = "_Service")]
/// This struct is implementation of the Apollo Federation subgraph specification.
/// see: https://www.apollographql.com/docs/federation/subgraph-spec/
struct Service {
    /// Represents the schema of the subgraph. It is a short form of the schema definition language (SDL).
    sdl: String,
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(name = "_service")]
    /// This resolver supports the enhanced introspection query for Apollo Federation.
    fn _service() -> Service {
        Service { sdl: get_schema() }
    }

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

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_actor(context: &Context, name: String, role: Role) -> Actor {
        sqlx::query!(
            r#"INSERT INTO actors (name, role) VALUES ($1, $2)"#,
            name,
            role as Role
        )
        .execute(&context.pool)
        .await
        .unwrap();

        let actor = sqlx::query_as!(
            Actor,
            r#"SELECT id, name, role as "role!: Role" FROM actors WHERE name = $1"#,
            name
        )
        .fetch_one(&context.pool)
        .await
        .unwrap();

        actor
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}

pub fn get_schema() -> String {
    let schema = create_schema();
    schema.as_schema_language()
}
