use super::actor::Actor;
use crate::context::Context;
use crate::schema::role::Role;
use chrono::TimeZone;

pub(crate) const POST_ID_PREFIX: &str = "Post:";

#[derive(Debug, Clone)]
pub(crate) struct Post {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) actor_id: i32,
    pub(crate) created_at: chrono::NaiveDateTime,
    pub(crate) updated_at: chrono::NaiveDateTime,
}

#[juniper::graphql_object(Context = Context)]
impl Post {
    fn id(&self) -> juniper::ID {
        format!("{POST_ID_PREFIX}{}", self.id).into()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    async fn author(&self, context: &Context) -> juniper::FieldResult<Actor> {
        let actor = sqlx::query_as!(
            Actor,
            r#"SELECT id, name, role as "role!: Role", created_at, updated_at FROM actors WHERE id = $1"#,
            self.actor_id
        )
        .fetch_one(&context.pool)
        .await?;

        Ok(actor)
    }

    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_local_datetime(&self.created_at).unwrap()
    }

    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc.from_local_datetime(&self.updated_at).unwrap()
    }
}

impl juniper_relay_connection::RelayConnectionNode for Post {
    type Cursor = String;

    fn cursor(&self) -> Self::Cursor {
        format!("{POST_ID_PREFIX}{}", self.id)
    }

    fn connection_type_name() -> &'static str {
        "PostConnection"
    }

    fn edge_type_name() -> &'static str {
        "PostConnectionEdge"
    }
}

pub(crate) async fn posts_connection(
    context: &Context,
    first: Option<i32>,
    after: Option<String>,
    last: Option<i32>,
    before: Option<String>,
) -> juniper::FieldResult<juniper_relay_connection::RelayConnection<Post>> {
    juniper_relay_connection::RelayConnection::<Post>::new_async(
        first,
        after,
        last,
        before,
        |after, before, limit| async move {
            let after = if let Some(after) = after {
                if after.starts_with(POST_ID_PREFIX) {
                    after.trim_start_matches(POST_ID_PREFIX).parse::<i32>()?
                } else {
                    0
                }
            } else {
                0
            };
            let before = if let Some(before) = before {
                if before.starts_with(POST_ID_PREFIX) {
                    before.trim_start_matches(POST_ID_PREFIX).parse::<i32>()?
                } else {
                    0
                }
            } else {
                i32::MAX
            };

            let query = sqlx::query_as!(
                Post,
                r#"
            SELECT id, title, actor_id, created_at, updated_at
            FROM posts
            WHERE id > $1 AND id < $2
            ORDER BY id ASC
            LIMIT $3
            "#,
                after,
                before,
                limit.unwrap_or(10)
            );

            let posts = query.fetch_all(&context.pool).await?;

            Ok(posts)
        },
    )
    .await
}
