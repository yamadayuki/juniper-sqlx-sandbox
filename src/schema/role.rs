#[derive(juniper::GraphQLEnum, Debug, Clone, Copy, sqlx::Type)]
#[graphql(description = "Represents a user role")]
#[sqlx(type_name = "actor_role", rename_all = "lowercase")]
pub(crate) enum Role {
    Admin,
    Editor,
    Viewer,
}
