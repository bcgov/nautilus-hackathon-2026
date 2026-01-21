delete from deployment;
delete from pipeline;
delete from repository;

insert into repository (id, name, url, created_at, updated_at)
values (1, 'nautilus', 'https://github.com/nautilus/nautilus', '2026-01-20T10:00:00Z', '2026-01-20T10:00:00Z');

insert into repository (id, name, url, created_at, updated_at)
values (2, 'harbor', 'https://github.com/nautilus/harbor', '2026-01-20T10:02:00Z', '2026-01-20T10:02:00Z');

insert into repository (id, name, url, created_at, updated_at)
values (3, 'seaglass', 'https://github.com/nautilus/seaglass', '2026-01-20T10:03:00Z', '2026-01-20T10:03:00Z');

insert into pipeline (id, repository_id, name, branch_name, auto_deploy, created_at, updated_at)
values (1, 1, 'nautilus-main', 'main', 1, '2026-01-20T10:05:00Z', '2026-01-20T10:05:00Z');

insert into pipeline (id, repository_id, name, branch_name, auto_deploy, created_at, updated_at)
values (2, 1, 'nautilus-release', 'test', 0, '2026-01-20T10:06:00Z', '2026-01-20T10:06:00Z');

insert into pipeline (id, repository_id, name, branch_name, auto_deploy, created_at, updated_at)
values (3, 2, 'harbor-main', 'main', 1, '2026-01-20T10:07:00Z', '2026-01-20T10:07:00Z');

insert into deployment (id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id)
values (1, 1, 'abc123def456', 'successful', '2026-01-20T10:10:00Z', '2026-01-20T10:11:00Z', '2026-01-20T10:12:00Z', 42);

insert into deployment (id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id)
values (2, 1, 'fedcba654321', 'failed', '2026-01-20T11:10:00Z', '2026-01-20T11:11:00Z', '2026-01-20T11:12:00Z', null);

insert into deployment (id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id)
values (3, 3, null, 'pending', '2026-01-20T12:10:00Z', null, null, null);

-- Add data for our nautilus test repo

insert into repository (id, name, url, created_at, updated_at)
values (99, 'demo-repo', 'https://github.com/bcgov/nautilus-test-repo', '2026-01-20T10:00:00Z', '2026-01-20T10:00:00Z');

insert into pipeline (id, repository_id, name, branch_name, auto_deploy, created_at, updated_at)
values (99, 99, 'demo-repo-pipeline', 'main', 0, '2026-01-20T10:05:00Z', '2026-01-20T10:05:00Z');

insert into deployment (id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id)
values (99, 99, '8534eac40f98f8680c216293e7c8c943d24142bb', 'successful', '2026-01-20T10:10:00Z', '2026-01-20T10:11:00Z', '2026-01-20T10:12:00Z', 42);
