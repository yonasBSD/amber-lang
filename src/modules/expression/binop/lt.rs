use heraclitus_compiler::prelude::*;
use crate::modules::prelude::*;
use crate::modules::expression::expr::Expr;
use crate::translate::compare::{translate_lexical_comparison, translate_array_lexical_comparison, ComparisonOperator};
use crate::translate::compute::{translate_float_computation, ArithOp};
use crate::modules::types::{Typed, Type};
use super::BinOp;

#[derive(Debug, Clone)]
pub struct Lt {
    left: Box<Expr>,
    right: Box<Expr>
}

impl Typed for Lt {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl BinOp for Lt {
    fn set_left(&mut self, left: Expr) {
        self.left = Box::new(left);
    }

    fn set_right(&mut self, right: Expr) {
        self.right = Box::new(right);
    }

    fn parse_operator(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "<")?;
        Ok(())
    }
}

impl SyntaxModule<ParserMetadata> for Lt {
    syntax_name!("Lt");

    fn new() -> Self {
        Lt {
            left: Box::new(Expr::new()),
            right: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        Self::typecheck_allowed_types(meta, "comparison", &self.left, &self.right, &[
            Type::Num,
            Type::Int,
            Type::Text,
            Type::array_of(Type::Num),
            Type::array_of(Type::Int),
            Type::array_of(Type::Text),
        ])?;
        Ok(())
    }
}

impl TranslateModule for Lt {
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        match self.left.get_type() {
            Type::Int => {
                let left = self.left.translate(meta).with_quotes(false);
                let right = self.right.translate(meta).with_quotes(false);
                ArithmeticFragment::new(left, ArithOp::Lt, right).to_frag()
            }
            Type::Num => {
                let left = self.left.translate(meta);
                let right = self.right.translate(meta);
                translate_float_computation(meta, ArithOp::Lt, Some(left), Some(right))
            }
            Type::Array(inner_type) => {
                translate_array_lexical_comparison(meta, ComparisonOperator::Lt, &self.left, &self.right, *inner_type)
            }
            Type::Text => {
                translate_lexical_comparison(meta, ComparisonOperator::Lt, &self.left, &self.right)
            }
            _ => unreachable!("Unsupported type {} in less than comparison", self.left.get_type())
        }
    }
}

impl DocumentationModule for Lt {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
