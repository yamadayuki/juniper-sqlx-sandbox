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

pub(crate) async fn get_actor(context: &Context, id: i32) -> juniper::FieldResult<Option<Actor>> {
    let actor = sqlx::query_as!(
        Actor,
        r#"SELECT id, name, role as "role!: Role" FROM actors WHERE id = $1"#,
        id
    )
    .fetch_optional(&context.pool)
    .await?;

    Ok(actor)
}

#[derive(juniper::GraphQLInputObject)]
pub(crate) struct CreateActorInput {
    name: String,
    role: Role,
}

pub(crate) async fn create_actor(
    context: &Context,
    new_actor: CreateActorInput,
) -> juniper::FieldResult<Actor> {
    let actor = sqlx::query_as!(
        Actor,
        r#"INSERT INTO actors (name, role) VALUES ($1, $2) RETURNING id, name, role as "role!: Role""#,
        new_actor.name,
        new_actor.role as Role
    )
    .fetch_one(&context.pool)
    .await?;

    Ok(actor)
}
