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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    users,
);
