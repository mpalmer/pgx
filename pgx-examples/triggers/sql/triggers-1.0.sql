/* 
This file is auto generated by pgx.

The ordering of items is not stable, it is driven by a dependency graph.
*/

-- src/lib.rs:11
-- triggers::trigger_example
CREATE OR REPLACE FUNCTION "trigger_example"() RETURNS trigger
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'trigger_example_wrapper';

-- src/lib.rs:49

CREATE TABLE test (
    id serial8 NOT NULL PRIMARY KEY,
    title varchar(50),
    description text,
    payload jsonb
);

CREATE TRIGGER test_trigger BEFORE INSERT ON test FOR EACH ROW EXECUTE PROCEDURE trigger_example();
INSERT INTO test (title, description, payload) VALUES ('the title', 'a description', '{"key": "value"}');

