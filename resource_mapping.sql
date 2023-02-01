CREATE TABLE pogo_resource_mapping(
    task_id uuid,
    resource_id uuid
);
CREATE INDEX resource_map_index ON pogo_resource_mapping (task_id, resource_id);
