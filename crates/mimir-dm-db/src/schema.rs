// @generated automatically by Diesel CLI.

diesel::table! {
    campaigns (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        settings -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// Note: embeddings table is a sqlite-vec virtual table and not managed by Diesel
// It will be accessed directly through rusqlite for vector operations

diesel::table! {
    npcs (id) {
        id -> Text,
        campaign_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        personality -> Nullable<Text>,
        relationships -> Nullable<Text>,
        stats -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    plots (id) {
        id -> Text,
        campaign_id -> Text,
        title -> Text,
        summary -> Nullable<Text>,
        status -> Text,
        connections -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rules (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        category -> Text,
        source -> Nullable<Text>,
        page_reference -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        campaign_id -> Text,
        session_number -> Integer,
        date -> Date,
        summary -> Nullable<Text>,
        notes -> Nullable<Text>,
        participants -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(npcs -> campaigns (campaign_id));
diesel::joinable!(plots -> campaigns (campaign_id));
diesel::joinable!(sessions -> campaigns (campaign_id));

diesel::allow_tables_to_appear_in_same_query!(
    campaigns,
    npcs,
    plots,
    rules,
    sessions,
);