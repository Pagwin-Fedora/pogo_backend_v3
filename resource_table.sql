CREATE TABLE pogo_resources(
    id uuid DEFAULT gen_random_uuid(),
    location varchar
);
CREATE UNIQUE INDEX resouce_index ON pogo_resources (id);
