#[derive(Clone, Copy)]
pub(crate) enum Token<'b> {
    Integer(i64),
    Bytes(&'b [u8]),
    List,
    Dictionary,
    End,
}
