CREATE TABLE pogo_relations (
    parent uuid,
    child uuid
);

CREATE INDEX task_relation_index ON pogo_relations(parent, child);
