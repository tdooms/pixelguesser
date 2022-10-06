create table if not exists users
(
    pw_hash  text not null,
    username text,
    role     integer,
    email    text not null unique,
    refresh  text
)