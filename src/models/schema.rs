// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 64]
        symbol -> Varchar,
        #[max_length = 64]
        owner -> Varchar,
        #[max_length = 255]
        pic_url -> Varchar,
        #[max_length = 64]
        contract_address -> Varchar,
        chain_id -> Int4,
        #[max_length = 64]
        dir_name -> Varchar,
        #[max_length = 64]
        dir_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    nft_traits (id) {
        id -> Uuid,
        nft_id -> Uuid,
        #[max_length = 255]
        trait_type -> Varchar,
        #[max_length = 255]
        trait_value -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    nfts (id) {
        id -> Uuid,
        token_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        image_url -> Text,
        supply -> Int4,
        #[max_length = 255]
        external_link -> Nullable<Varchar>,
        #[max_length = 64]
        owner -> Varchar,
        #[max_length = 64]
        collection -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 64]
        address -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    collections,
    nft_traits,
    nfts,
    users,
);
