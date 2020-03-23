#[derive(Debug, Copy, Clone)]
pub enum Object {
    Reference(u64),
    Unit
}