table! {
    dailystats (date) {
        date -> Timestamp,
        pastecount -> Int4,
        pasteviews -> Int4,
    }
}

table! {
    pastes (pasteid) {
        pasteid -> Varchar,
        token -> Varchar,
        lexer -> Varchar,
        expiration -> Timestamp,
        burn -> Int4,
        paste -> Text,
        paste_lexed -> Nullable<Text>,
        size -> Nullable<Int4>,
        lines -> Nullable<Int4>,
        sloc -> Nullable<Int4>,
    }
}

table! {
    stats (metric) {
        metric -> Varchar,
        counter -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    dailystats,
    pastes,
    stats,
);
