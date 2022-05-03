use diesel::backend::Backend;
use diesel::pg::Pg;
use diesel::prelude::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

table! {
    posts {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(users, posts);

#[derive(Queryable, Identifiable, AsChangeset, Selectable, Debug)]
struct User {
    id: i32,
    name: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = posts)]
struct JustPostTitleAndId {
    id: i32,
    title: String,
}

/// works!
fn do_thing_complex_query(
    conn: &mut impl Connection<Backend = Pg>,
) -> QueryResult<(User, JustPostTitleAndId)> {
    users::table
        .inner_join(posts::table.on(users::id.eq(posts::user_id)))
        .select((User::as_select(), JustPostTitleAndId::as_select()))
        .first(conn)
}

/// works too!
fn do_thing_complex_query_with_alias(
    conn: &mut impl Connection<Backend = Pg>,
) -> QueryResult<(User, JustPostTitleAndId, JustPostTitleAndId)> {
    let posts_alias = diesel::alias!(posts as posts_alias);
    users::table
        .inner_join(posts::table.on(users::id.eq(posts::user_id)))
        .inner_join(posts_alias.on(users::id.eq(posts_alias.field(posts::user_id))))
        .select((
            User::as_select(),
            JustPostTitleAndId::as_select(),
            posts_alias.fields(<JustPostTitleAndId as Selectable<Pg>>::construct_selection()),
        ))
        .first(conn)
}
