create table users (
    user_id serial primary key,

    nickname text unique not null,
    email text unique not null,

    last_seen timestamp with time zone not null default now(),
    verified boolean not null default false,
    image jsonb
);

create table quizzes (
    quiz_id serial primary key,

    public boolean not null default false,
    title text not null,
    description text not null,
    explanation text not null,

    creator_id integer not null references users(user_id),
    created_at timestamp with time zone not null default now(),

    complete boolean not null default false,
    image jsonb
);

create table tags (
    quiz_id integer not null references quizzes(quiz_id) on delete cascade,
    value text not null,

    primary key (quiz_id, value)
);

create table sessions (
    session_id serial primary key,
    quiz_id integer not null references quizzes(quiz_id)
);

create table rounds (
    round_index integer not null,
    quiz_id integer not null references quizzes(quiz_id) on delete cascade,

    points integer not null,
    guesses integer not null,
    answer text not null,

    speed integer not null default 100,
    algorithm integer not null default 0,

    image jsonb,

    primary key (round_index, quiz_id)
);