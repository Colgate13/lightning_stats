// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (public_key) {
        #[max_length = 66]
        public_key -> Varchar,
        #[max_length = 255]
        alias -> Varchar,
        capacity -> Numeric,
        first_seen -> Timestamptz,
    }
}
