create extension if not exists "uuid-ossp";

create collation case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);

create table users (
    user_id uuid primary key default uuid_generate_v4(),
    username text collate "case_insensitive" unique not null
);

create table follows (
    following_user_id uuid not null references users(user_id) on delete cascade,
    followed_user_id uuid not null references users(user_id) on delete cascade,
    primary key (following_user_id, followed_user_id)
);