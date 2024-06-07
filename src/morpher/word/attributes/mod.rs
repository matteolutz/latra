use super::WordType;

pub mod noun;
pub mod verb;

pub trait WordAttributesTrait<T> {
    fn combine(others: &[T]) -> T;
    fn try_merge_into(&mut self, other: &T) -> Option<()>;
    fn word_type() -> WordType;
}
