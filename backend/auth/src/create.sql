create table if not exists users (
    pw_hash text not null,
    username text,
    email text not null unique
)