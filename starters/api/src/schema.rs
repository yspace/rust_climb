table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}

table! {
    product (product_id) {
        product_id -> Bpchar,
        product_name -> Varchar,
        product_type -> Varchar,
        sale_price -> Nullable<Int4>,
        purchase_price -> Nullable<Int4>,
        registse__datae -> Nullable<Date>,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    product,
);
