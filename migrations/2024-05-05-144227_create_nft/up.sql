-- Your SQL goes here
CREATE TABLE nfts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id INT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    image_url TEXT NOT NULL,
    supply INT NOT NULL,
    external_link VARCHAR(255),
    owner VARCHAR(64) NOT NULL,
    collection VARCHAR(64) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (token_id, collection)
);  