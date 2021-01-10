table! {
    check_point (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        creation_time -> Timestamptz,
        deactivation_time -> Nullable<Timestamptz>,
        precedence -> Int4,
        subject_id -> Int4,
    }
}

table! {
    subject (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        creation_time -> Timestamptz,
        deactivation_time -> Nullable<Timestamptz>,
    }
}

joinable!(check_point -> subject (subject_id));

allow_tables_to_appear_in_same_query!(
    check_point,
    subject,
);
