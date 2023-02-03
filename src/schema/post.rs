#[derive(juniper::GraphQLObject, Debug, Clone)]
pub(crate) struct Post {
    id: i32,
    title: String,
}
