pub enum Column<'a> {
    Pkey,
    Text(&'a str),
    Number(&'a str),
}
