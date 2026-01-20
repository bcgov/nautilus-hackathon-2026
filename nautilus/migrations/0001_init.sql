create table if not exists deployment (
    id integer primary key autoincrement,
    pipeline_id integer not null,
    commit_sha text,
    status text not null,
    created_at text not null,
    started_at text,
    ended_at text,
    pr_id integer,
    foreign key (pipeline_id) references pipeline (id),
    check (status in ('pending', 'in_progress', 'successful', 'failed', 'canceled', 'timed_out'))
);

create table if not exists pipeline (
    id integer primary key autoincrement,
    repository_id integer not null,
    name text not null unique,
    auto_deploy integer not null,
    created_at text not null,
    updated_at text not null,
    foreign key (repository_id) references repository (id)
);

create table if not exists repository (
    id integer primary key autoincrement,
    name text not null unique,
    url text not null,
    created_at text not null,
    updated_at text not null
);

