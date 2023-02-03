use crate::schema::actor::Actor;
use crate::{context::Context, schema::actor::CreateActorInput};
use juniper::{EmptySubscription, FieldResult, RootNode};

#[derive(juniper::GraphQLObject, Debug, Clone)]
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
    fn _service() -> FieldResult<Service> {
        Ok(Service { sdl: get_sdl() })
    }

    async fn actor(
        context: &Context,
        #[graphql(desc = "ID of the actor")] id: i32,
    ) -> FieldResult<Option<Actor>> {
        crate::schema::actor::get_actor(context, id).await
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_actor(context: &Context, new_actor: CreateActorInput) -> FieldResult<Actor> {
        crate::schema::actor::create_actor(context, new_actor).await
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn get_sdl() -> String {
    let schema = create_schema();
    schema.as_schema_language()
}
