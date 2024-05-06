-- Your SQL goes here
CREATE TABLE collections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    symbol VARCHAR(64) NOT NULL,
    owner VARCHAR(64) NOT NULL,
    pic_url VARCHAR(255) NOT NULL,
    contract_address VARCHAR(64) NOT NULL UNIQUE,
    chain_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);