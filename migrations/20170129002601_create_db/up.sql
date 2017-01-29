CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    plan_id INT

);

CREATE TABLE usr (
    id SERIAL PRIMARY KEY,
    email VARCHAR,
    first_name VARCHAR,
    last_name VARCHAR,
    account_id INT references account(id)
);


CREATE TABLE usr_secure (
    id SERIAL PRIMARY KEY references usr(id),
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    salt VARCHAR NOT NULL
);

CREATE TABLE billing (
    id SERIAL PRIMARY KEY,
    usr_id SERIAL NOT NULL references usr(id)
    external_ref INT
);