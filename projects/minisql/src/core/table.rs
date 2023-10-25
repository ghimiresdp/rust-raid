use super::{db::Database, dtype::DataTypes};

pub(crate) struct Column{
    dtype: DataTypes,
    nullable: bool,
    unique: bool,
    auto: bool,
    primary_key: bool,
}

pub(crate) struct Table {
    database: Database,
    columns: Vec<DataTypes>,
    constrains: Vec<String>
}
