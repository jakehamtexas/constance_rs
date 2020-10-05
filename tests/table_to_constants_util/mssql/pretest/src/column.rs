pub enum Column<'a> {
    Pkey,
    Text(&'a str),
    Number(&'a str),
}

pub static ID_COLUMN: Column = Column::Pkey;
pub static NAME_COLUMN: Column = Column::Text("name");
pub static DESCRIPTION_COLUMN: Column = Column::Text("description");
