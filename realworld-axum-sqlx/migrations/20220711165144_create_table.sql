create extension if not exists "uuid-ossp";

create collation case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);

create table "user" (
    user_id uuid primary key default uuid_generate_v4(),
    username text collate "case_insensitive" unique not null
)