-- Your SQL goes here
CREATE TABLE nft_traits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nft_id UUID NOT NULL,
    trait_type VARCHAR(255) NOT NULL,
    trait_value VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (nft_id, trait_type, trait_value)
);