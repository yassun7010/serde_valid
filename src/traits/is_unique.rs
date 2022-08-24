use itertools::Itertools;

pub trait IsUnique {
    fn is_unique(&self) -> bool;
}

impl<T> IsUnique for [T]
where
    T: std::cmp::Eq + std::hash::Hash,
{
    fn is_unique(&self) -> bool {
        let len = self.len();
        let unique = self.iter().unique();
        let (lower, upper) = unique.size_hint();
        if let Some(upper) = upper {
            if lower == len && upper == len {
                return true;
            }
        }
        unique.count() == len
    }
}
