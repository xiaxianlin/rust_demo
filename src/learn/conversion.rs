#[derive(Debug, Clone, Copy)]
pub struct IanaAllocated(pub u64);

impl From<u64> for IanaAllocated {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub fn is_iana_reserved<T>(s: T) -> bool
where
    T: Into<IanaAllocated>,
{
    let s = s.into();
    s.0 == 0 || s.0 == 65535
}
