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

diesel::joinable!(sources -> rule_systems (rule_system_id));

diesel::allow_tables_to_appear_in_same_query!(
    rule_systems,
    sources,
);