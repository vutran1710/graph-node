ALTER TABLE public.deployment_schemas
    ADD COLUMN IF NOT EXISTS subgraph_name text;

WITH t AS (
    SELECT
        subgraphs.subgraph.name AS name,
        subgraphs.subgraph_version.deployment AS HASH
    FROM
        subgraphs.subgraph_version
        INNER JOIN subgraphs.subgraph ON (subgraphs.subgraph_version.id = subgraphs.subgraph.current_version
                OR subgraphs.subgraph_version.id = subgraphs.subgraph.pending_version)
)
UPDATE
    public.deployment_schemas
SET
    subgraph_name = t.name
FROM
    t
WHERE
    subgraph = t.hash;
