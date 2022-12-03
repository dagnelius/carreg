// @generated automatically by Diesel CLI.

diesel::table! {
    ar_auto (id) {
        id -> Integer,
        ar_pase_id -> Integer,
        ar_modelis_id -> Integer,
        vin -> Varchar,
        tipa_apstiprinajuma_nr -> Varchar,
        ar_krasa_id -> Integer,
        sedvietas -> Integer,
        ar_veids_id -> Integer,
        piezimes -> Nullable<Varchar>,
    }
}

diesel::table! {
    ar_degviela (id) {
        id -> Integer,
        degviela -> Varchar,
    }
}

diesel::table! {
    ar_krasa (id) {
        id -> Integer,
        krasa -> Varchar,
    }
}

diesel::table! {
    ar_marka (id) {
        id -> Integer,
        marka -> Varchar,
    }
}

diesel::table! {
    ar_modelis (id) {
        id -> Integer,
        ar_marka_id -> Integer,
        modelis -> Varchar,
    }
}

diesel::table! {
    ar_motors (id) {
        id -> Integer,
        motora_tilpums -> Integer,
        ar_degviela_id -> Integer,
    }
}

diesel::table! {
    ar_pase (id) {
        id -> Integer,
        apliecibas_nr -> Varchar,
        registracijas_nr -> Varchar,
        datums_no -> Date,
        ar_turetaja_adrese_id -> Integer,
    }
}

diesel::table! {
    ar_turetaja_adrese (id) {
        id -> Integer,
        pilseta -> Varchar,
        iela -> Varchar,
    }
}

diesel::table! {
    ar_veids (id) {
        id -> Integer,
        veids -> Varchar,
    }
}

diesel::joinable!(ar_auto -> ar_krasa (ar_krasa_id));
diesel::joinable!(ar_auto -> ar_modelis (ar_modelis_id));
diesel::joinable!(ar_auto -> ar_pase (ar_pase_id));
diesel::joinable!(ar_auto -> ar_veids (ar_veids_id));
diesel::joinable!(ar_modelis -> ar_marka (ar_marka_id));
diesel::joinable!(ar_motors -> ar_degviela (ar_degviela_id));
diesel::joinable!(ar_pase -> ar_turetaja_adrese (ar_turetaja_adrese_id));

diesel::allow_tables_to_appear_in_same_query!(
    ar_auto,
    ar_degviela,
    ar_krasa,
    ar_marka,
    ar_modelis,
    ar_motors,
    ar_pase,
    ar_turetaja_adrese,
    ar_veids,
);
