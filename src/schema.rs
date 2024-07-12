// @generated automatically by Diesel CLI.

diesel::table! {
    user (user_id) {
        #[max_length = 255]
        user_id -> Varchar,
        human_diary -> Nullable<Text>,
        ai_diary_1 -> Nullable<Text>,
        ai_diary_2 -> Nullable<Text>,
        ai_diary_3 -> Nullable<Text>,
        ai_diary_4 -> Nullable<Text>,
        is_public -> Nullable<Bool>,
        favorite_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
