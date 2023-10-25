pub(crate) enum DataTypes {
    BOOL,
    INT,
    VARCHAR(u32),
    TEXT,
}

pub(crate) enum Data {
    BOOL(bool),
    INT(i128),
    VARCHAR(String),
    TEXT(String),
}
