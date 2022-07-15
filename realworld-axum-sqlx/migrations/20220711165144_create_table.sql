create extension if not exists "uuid-ossp";

create table "user" (
    user_id uuid primary key default uuid_generate_v4()
)