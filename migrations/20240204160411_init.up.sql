-- Add up migration script here
-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS sarcini (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        nume_sarcina VARCHAR(255) NOT NULL,
        notita_sarcina TEXT NOT NULL,
        ora_sarcina VARCHAR(100),
        data_sarcina VARCHAR(255),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
