CREATE TABLE IF NOT EXISTS cats (
    id serial PRIMARY KEY,
    name VARCHAR (140) not null,
    age smallint not null,
    weight real,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
); 