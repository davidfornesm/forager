pub(crate) enum Token<'b> {
    Number(i64),
    Bytes(&'b [u8]),
    List,
    Dictionary,
    End,
}
