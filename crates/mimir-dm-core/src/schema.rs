// @generated automatically by Diesel CLI.
diesel::table! {
    campaigns (id) {
        id -> Integer,
        name -> Text,
        status -> Text,
        directory_path -> Text,
        created_at -> Text,
        session_zero_date -> Nullable<Text>,
        first_session_date -> Nullable<Text>,
        last_activity_at -> Text,
    }
}

diesel::table! {
    modules (id) {
        id -> Integer,
        campaign_id -> Integer,
        name -> Text,
        module_number -> Integer,
        status -> Text,
        expected_sessions -> Integer,
        actual_sessions -> Integer,
        created_at -> Text,
        started_at -> Nullable<Text>,
        completed_at -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
        campaign_id -> Integer,
        module_id -> Nullable<Integer>,
        session_number -> Integer,
        status -> Text,
        scheduled_date -> Nullable<Text>,
        prep_started_at -> Nullable<Text>,
        prep_completed_at -> Nullable<Text>,
        completed_at -> Nullable<Text>,
        created_at -> Text,
    }
}

diesel::table! {
    workflow_cards (id) {
        id -> Text,
        board_type -> Text,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Text,
        last_moved_at -> Text,
        workflow_state -> Text,
        campaign_id -> Nullable<Integer>,
        module_id -> Nullable<Integer>,
        session_id -> Nullable<Integer>,
        priority -> Integer,
    }
}

diesel::table! {
    workflow_card_tags (card_id, tag) {
        card_id -> Text,
        tag -> Text,
    }
}

diesel::table! {
    template_documents (document_id, version_number) {
        document_id -> Text,
        version_number -> Integer,
        document_content -> Text,
        content_hash -> Text,
        document_type -> Nullable<Text>,
        document_level -> Nullable<Text>,
        purpose -> Nullable<Text>,
        variables_schema -> Nullable<Text>,
        default_values -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        is_active -> Bool,
        metadata -> Nullable<Text>,
    }
}

diesel::table! {
    documents (id) {
        id -> Integer,
        campaign_id -> Integer,
        module_id -> Nullable<Integer>,
        session_id -> Nullable<Integer>,
        template_id -> Text,
        document_type -> Text,
        title -> Text,
        file_path -> Text,
        created_at -> Text,
        updated_at -> Text,
        completed_at -> Nullable<Text>,
    }
}

diesel::joinable!(modules -> campaigns (campaign_id));
diesel::joinable!(sessions -> campaigns (campaign_id));
diesel::joinable!(sessions -> modules (module_id));
diesel::joinable!(workflow_cards -> campaigns (campaign_id));
diesel::joinable!(workflow_cards -> modules (module_id));
diesel::joinable!(workflow_cards -> sessions (session_id));
diesel::joinable!(workflow_card_tags -> workflow_cards (card_id));
diesel::joinable!(documents -> campaigns (campaign_id));
diesel::joinable!(documents -> modules (module_id));
diesel::joinable!(documents -> sessions (session_id));

diesel::allow_tables_to_appear_in_same_query!(
    campaigns,
    modules,
    sessions,
    workflow_cards,
    workflow_card_tags,
    template_documents,
    documents,
);