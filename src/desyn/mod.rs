pub mod ungeneric;
pub mod unpath;
pub mod untype;

pub trait DeSyntax {
    fn de_syntax(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
