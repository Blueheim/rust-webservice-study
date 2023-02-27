CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS accounts (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(120) NOT NULL,
        verified BOOLEAN NOT NULL DEFAULT FALSE,
        role VARCHAR(50) NOT NULL DEFAULT 'member',
        created_on TIMESTAMP NOT NULL DEFAULT NOW(),
        updated_on TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

CREATE INDEX accounts_email_idx ON accounts (email);