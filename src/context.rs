use crate::db::Pool;

pub struct Context {
    pub pool: Pool,
}

impl juniper::Context for Context {}
