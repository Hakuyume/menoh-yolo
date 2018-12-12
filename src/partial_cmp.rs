use std::cmp;

pub fn min<T>(v1: T, v2: T) -> Option<T>
where
    T: cmp::PartialOrd,
{
    match v1.partial_cmp(&v2)? {
        cmp::Ordering::Greater => Some(v2),
        _ => Some(v1),
    }
}

pub fn max<T>(v1: T, v2: T) -> Option<T>
where
    T: cmp::PartialOrd,
{
    match v1.partial_cmp(&v2)? {
        cmp::Ordering::Less => Some(v2),
        _ => Some(v1),
    }
}
