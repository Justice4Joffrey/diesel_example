use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{ConnectionManager, PooledConnection, Pool};
use diesel::prelude::*;

/// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

/// A trait to allow shorthand generics over pg connections
pub trait PostgresConnection: Connection<Backend=Pg> {}

impl PostgresConnection for PgConnection {}

impl PostgresConnection for PooledConnection<ConnectionManager<PgConnection>> {}


table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

#[derive(Queryable, Identifiable, AsChangeset, Selectable, Debug)]
struct User {
    id: i32,
    name: String,
}

fn do_thing(conn: &mut impl PostgresConnection) -> QueryResult<User> {
    users::table.select(User::as_select()).first(conn)
}


fn do_thing_must_use_verbose_trait_bounds(conn: &mut impl PostgresConnection) -> QueryResult<()> {
    let mut user = users::table.select(User::as_select()).first(conn)?;
    user.name = "test".to_owned();
    user.save_changes(conn)
}
