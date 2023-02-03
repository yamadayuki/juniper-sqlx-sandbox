use super::post::Post;
use super::role::Role;
use crate::context::Context;
use chrono::TimeZone;

const ACTOR_ID_PREFIX: &str = "Actor:";

#[derive(Debug)]
pub(crate) struct Actor {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) role: Role,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) updated_at: chrono::NaiveDateTime,
}

#[juniper::graphql_object(Context = Context)]
impl Actor {
    fn id(&self) -> juniper::ID {
        format!("{ACTOR_ID_PREFIX}:{}", self.id).into()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn role(&self) -> Role {
        self.role
    }

    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_local_datetime(&self.created_at).unwrap()
    }

    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_local_datetime(&self.updated_at).unwrap()
    }

    // custom resolvers

    fn posts(&self) -> Vec<Post> {
        // self.posts.clone()
        vec![]
    }
}

pub(crate) async fn get_actor(
    context: &Context,
    id: juniper::ID,
) -> juniper::FieldResult<Option<Actor>> {
    if !id.starts_with(ACTOR_ID_PREFIX) {
        return Ok(None);
    }

    let id = id.trim_start_matches(ACTOR_ID_PREFIX).parse::<i32>()?;

    let actor = sqlx::query_as!(
        Actor,
        r#"SELECT id, name, role as "role!: Role", created_at, updated_at FROM actors WHERE id = $1"#,
        id
    )
    .fetch_optional(&context.pool)
    .await?;

    dbg!(&actor);

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
        r#"
        INSERT INTO actors (name, role)
        VALUES ($1, $2)
        RETURNING id, name, role as "role!: Role", created_at, updated_at
        "#,
        new_actor.name,
        new_actor.role as Role
    )
    .fetch_one(&context.pool)
    .await?;

    Ok(actor)
}

impl juniper_relay_connection::RelayConnectionNode for Actor {
    type Cursor = String;

    fn cursor(&self) -> Self::Cursor {
        format!("{ACTOR_ID_PREFIX}:{}", self.id)
    }

    fn connection_type_name() -> &'static str {
        "ActorConnection"
    }

    fn edge_type_name() -> &'static str {
        "ActorConnectionEdge"
    }
}

pub(crate) async fn actors_connection(
    context: &Context,
    first: Option<i32>,
    after: Option<String>,
    last: Option<i32>,
    before: Option<String>,
) -> juniper::FieldResult<juniper_relay_connection::RelayConnection<Actor>> {
    juniper_relay_connection::RelayConnection::<Actor>::new_async(
        first,
        after,
        last,
        before,
        |after, before, limit| async move {
            let after = if let Some(after) = after {
                if after.starts_with(ACTOR_ID_PREFIX) {
                    after.trim_start_matches(ACTOR_ID_PREFIX).parse::<i32>()?
                } else {
                    0
                }
            } else {
                0
            };
            let before = if let Some(before) = before {
                if before.starts_with(ACTOR_ID_PREFIX) {
                    before.trim_start_matches(ACTOR_ID_PREFIX).parse::<i32>()?
                } else {
                    0
                }
            } else {
                i32::MAX
            };

            let query = sqlx::query_as!(
                Actor,
                r#"
            SELECT id, name, role as "role!: Role", created_at, updated_at
            FROM actors
            WHERE id > $1 AND id < $2
            ORDER BY id ASC
            LIMIT $3
            "#,
                after,
                before,
                limit.unwrap_or(10)
            );

            let actors = query.fetch_all(&context.pool).await?;

            Ok(actors)
        },
    )
    .await
}
