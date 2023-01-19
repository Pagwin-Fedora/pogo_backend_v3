CREATE TABLE pogo_tasks (
    id uuid DEFAULT gen_random_uuid(),
    title varchar DEFAULT '',
    body varchar DEFAULT '',
    progress real DEFAULT 0.0,
    login varchar
);
CREATE INDEX task_index ON pogo_tasks (id, login);
