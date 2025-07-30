// @generated automatically by Diesel CLI.

diesel::table! {
    rule_systems (id) {
        id -> Text,
        name -> Text,
        short_name -> Nullable<Text>,
        publisher -> Nullable<Text>,
        version -> Nullable<Text>,
        is_active -> Bool,
        metadata -> Nullable<Text>,
    }
}

diesel::table! {
    sources (id) {
        id -> Text,
        rule_system_id -> Text,
        name -> Text,
        abbreviation -> Nullable<Text>,
        publisher -> Nullable<Text>,
        publish_date -> Nullable<Text>,
        is_official -> Bool,
        is_srd -> Bool,
        metadata -> Nullable<Text>,
    }
}

diesel::table! {
    races (id) {
        id -> Text,
        name -> Text,
        rule_system_id -> Text,
        source_id -> Text,
        page -> Nullable<Integer>,
        race_type -> Text,
        parent_race_id -> Nullable<Text>,
        size -> Nullable<Text>,
        speed -> Nullable<Text>,
        ability_scores -> Nullable<Text>,
        age -> Nullable<Text>,
        alignment_tendency -> Nullable<Text>,
        language_proficiencies -> Nullable<Text>,
        trait_tags -> Nullable<Text>,
        entries -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    classes (id) {
        id -> Text,
        name -> Text,
        rule_system_id -> Text,
        source_id -> Text,
        page -> Nullable<Integer>,
        class_type -> Text,
        parent_class_id -> Nullable<Text>,
        hit_die -> Nullable<Integer>,
        primary_abilities -> Nullable<Text>,
        saving_throws -> Nullable<Text>,
        skill_proficiency_count -> Nullable<Integer>,
        skill_proficiency_choices -> Nullable<Text>,
        starting_proficiencies -> Nullable<Text>,
        starting_equipment -> Nullable<Text>,
        spell_ability -> Nullable<Text>,
        caster_progression -> Nullable<Text>,
        subclass_title -> Nullable<Text>,
        subclass_level -> Nullable<Integer>,
        features -> Nullable<Text>,
        spell_slots -> Nullable<Text>,
        entries -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    items (id) {
        id -> Text,
        name -> Text,
        rule_system_id -> Text,
        source_id -> Text,
        page -> Nullable<Integer>,
        base_item_id -> Nullable<Text>,
        #[sql_name = "type"]
        item_type -> Nullable<Text>,
        weight_lb -> Nullable<Float>,
        value_cp -> Nullable<Integer>,
        armor_class -> Nullable<Integer>,
        damage -> Nullable<Text>,
        properties -> Nullable<Text>,
        rarity -> Nullable<Text>,
        requires_attunement -> Bool,
        attunement_prereq -> Nullable<Text>,
        magic_bonus -> Nullable<Integer>,
        additional_properties -> Nullable<Text>,
        entries -> Text,
        is_magic -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    backgrounds (id) {
        id -> Text,
        name -> Text,
        rule_system_id -> Text,
        source_id -> Text,
        page -> Nullable<Integer>,
        skill_proficiencies -> Nullable<Text>,
        language_proficiencies -> Nullable<Text>,
        tool_proficiencies -> Nullable<Text>,
        starting_equipment -> Nullable<Text>,
        feature_name -> Nullable<Text>,
        feature_text -> Nullable<Text>,
        entries -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    feats (id) {
        id -> Text,
        name -> Text,
        rule_system_id -> Text,
        source_id -> Text,
        page -> Nullable<Integer>,
        prerequisites -> Nullable<Text>,
        ability_increases -> Nullable<Text>,
        feat_type -> Nullable<Text>,
        entries -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(sources -> rule_systems (rule_system_id));
diesel::joinable!(races -> rule_systems (rule_system_id));
diesel::joinable!(races -> sources (source_id));
diesel::joinable!(classes -> rule_systems (rule_system_id));
diesel::joinable!(classes -> sources (source_id));
diesel::joinable!(items -> rule_systems (rule_system_id));
diesel::joinable!(items -> sources (source_id));
diesel::joinable!(backgrounds -> rule_systems (rule_system_id));
diesel::joinable!(backgrounds -> sources (source_id));
diesel::joinable!(feats -> rule_systems (rule_system_id));
diesel::joinable!(feats -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    rule_systems,
    sources,
    races,
    classes,
    items,
    backgrounds,
    feats,
);