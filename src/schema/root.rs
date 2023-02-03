use crate::context::Context;
use crate::schema::{actor::Actor, role::Role};
use juniper::{EmptySubscription, GraphQLObject, RootNode};

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(name = "_Service")]
/// This struct is implementation of the Apollo Federation subgraph specification.
/// see: https://www.apollographql.com/docs/federation/subgraph-spec/
struct Service {
    /// Represents the schema of the subgraph. It is a short form of the schema definition language (SDL).
    sdl: String,
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
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

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
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

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn get_schema() -> String {
    let schema = create_schema();
    schema.as_schema_language()
}
