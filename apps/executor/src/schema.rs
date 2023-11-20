// @generated automatically by Diesel CLI.

diesel::table! {
    mails (id) {
        id -> Uuid,
        app -> Varchar,
        subject -> Varchar,
        receiver_name -> Varchar,
        receiver_email -> Varchar,
        message -> Text,
        reply_to_name -> Nullable<Varchar>,
        reply_to_email -> Nullable<Varchar>,
        cc -> Nullable<Json>,
        bcc -> Nullable<Json>,
        sent_at -> Timestamp,
    }
}
