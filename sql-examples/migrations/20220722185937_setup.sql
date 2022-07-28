create extension if not exists "uuid-ossp";

create table expenses (
    id uuid primary key default uuid_generate_v4(),
    name varchar(10) not null,
    deposits integer not null,
    withdrawals integer not null
)