// @generated automatically by Diesel CLI.

diesel::table! {
    catalog_cults (id) {
        id -> Integer,
        name -> Text,
        category -> Text,
        cult_type -> Nullable<Text>,
        source -> Text,
        page -> Nullable<Integer>,
        full_cult_json -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
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
        archived_at -> Nullable<Text>,
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

diesel::table! {
    catalog_actions (id) {
        id -> Integer,
        name -> Text,
        time_type -> Text,
        description -> Text,
        see_also -> Nullable<Text>,
        source -> Text,
        full_action_json -> Text,
    }
}

diesel::table! {
    catalog_backgrounds (id) {
        id -> Integer,
        name -> Text,
        skills -> Text,
        languages -> Text,
        tools -> Text,
        feature -> Text,
        source -> Text,
        full_background_json -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    catalog_conditions (id) {
        id -> Integer,
        name -> Text,
        item_type -> Text,
        description -> Text,
        source -> Text,
        full_condition_json -> Text,
    }
}

diesel::table! {
    catalog_spells (id) {
        id -> Integer,
        name -> Text,
        level -> Integer,
        school -> Text,
        cast_time -> Text,
        range -> Text,
        components -> Text,
        tags -> Text,
        source -> Text,
        full_spell_json -> Text,
    }
}

diesel::table! {
    catalog_sources (source_name) {
        source_name -> Text,
        catalog_type -> Text,
        last_imported -> Nullable<Text>,
        file_path -> Text,
        file_hash -> Text,
        record_count -> Integer,
    }
}

diesel::table! {
    uploaded_books (id) {
        id -> Text,
        name -> Text,
        location -> Text,
        archive_path -> Text,
        uploaded_at -> Text,
        metadata_json -> Nullable<Text>,
    }
}

diesel::table! {
    catalog_languages (id) {
        id -> Integer,
        name -> Text,
        language_type -> Text,
        script -> Text,
        typical_speakers -> Text,
        source -> Text,
        full_language_json -> Text,
    }
}

diesel::table! {
    catalog_rewards (id) {
        id -> Integer,
        name -> Text,
        reward_type -> Text,
        description -> Text,
        has_prerequisites -> Integer,
        source -> Text,
        full_reward_json -> Text,
    }
}

diesel::table! {
    catalog_feats (id) {
        id -> Integer,
        name -> Text,
        prerequisites -> Nullable<Text>,
        brief -> Nullable<Text>,
        source -> Text,
        full_feat_json -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    catalog_races (id) {
        id -> Integer,
        name -> Text,
        size -> Nullable<Text>,
        speed -> Nullable<Integer>,
        ability_bonuses -> Nullable<Text>,
        traits_count -> Integer,
        source -> Text,
        full_race_json -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    catalog_objects (id) {
        id -> Integer,
        name -> Text,
        object_type -> Nullable<Text>,
        size -> Nullable<Text>,
        ac -> Nullable<Text>,
        hp -> Nullable<Text>,
        source -> Text,
        full_object_json -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    catalog_psionics (id) {
        id -> Integer,
        name -> Text,
        psionic_type -> Text,
        psionic_order -> Nullable<Text>,
        source -> Text,
        page -> Nullable<Integer>,
        full_psionic_json -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    catalog_traps (id) {
        id -> Integer,
        name -> Text,
        category -> Text,
        trap_type -> Nullable<Text>,
        source -> Text,
        full_trap_json -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    catalog_variant_rules (id) {
        id -> Integer,
        name -> Text,
        rule_type -> Nullable<Text>,
        source -> Text,
        page -> Nullable<Integer>,
        full_variant_rule_json -> Text,
        created_at -> Nullable<Timestamp>,
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
    catalog_actions,
    catalog_backgrounds,
    catalog_conditions,
    catalog_cults,
    catalog_feats,
    catalog_languages,
    catalog_objects,
    catalog_psionics,
    catalog_races,
    catalog_rewards,
    catalog_spells,
    catalog_sources,
    catalog_traps,
    catalog_variant_rules,
    uploaded_books,
);