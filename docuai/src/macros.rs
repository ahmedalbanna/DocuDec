#[macro_export]
macro_rules! string {
    () => {
        $crate::docu_requests::schema_builder::Schema::new(
            $crate::docu_requests::schema_builder::SchemaType::String,
        )
    };
}

#[macro_export]
macro_rules! array {
    ($type:expr) => {
        $crate::docu_requests::schema_builder::Schema::new(
            $crate::docu_requests::schema_builder::SchemaType::Array,
        )
        .items($type)
    };
}
