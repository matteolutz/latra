use super::WordType;

pub mod noun;
pub mod verb;

pub trait WordAttributes: std::fmt::Display + std::fmt::Debug {
    fn word_type(&self) -> WordType;
    fn as_any(&self) -> &dyn std::any::Any;
    fn eq_box(&self, other: &Box<dyn WordAttributes>) -> bool;
}
