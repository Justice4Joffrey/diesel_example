
Using SaveChangesDsl requires me to use this specific connection type.

```
>> cargo test

Compiling diesel_example v0.1.0 (/home/otto/projects/diesel_example)
error[E0277]: the trait bound `impl PostgresConnection: UpdateAndFetchResults<&User, _>` is not satisfied
   --> src/lib.rs:39:23
    |
39  |     user.save_changes(conn)
    |          ------------ ^^^^ the trait `UpdateAndFetchResults<&User, _>` is not implemented for `impl PostgresConnection`
    |          |
    |          required by a bound introduced by this call
    |
note: required by a bound in `save_changes`
   --> /home/otto/.cargo/git/checkouts/diesel-6e3331fb3b9331ec/b0a2c1e/diesel/src/query_dsl/save_changes_dsl.rs:146:15
    |
146 |         Conn: UpdateAndFetchResults<Self, T>,
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `save_changes`

```