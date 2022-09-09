use crate::expr::Expr;
use crate::utils;
use crate::utils::*;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
   name: String,
   val: Expr,
}

impl BindingDef {
   pub fn new(s: &str,) -> (&str, Self) {
      let s = utils::tag("rn", s);

      let (s, name) = utils::extract_identifier(s);
      let (s, _) = extract_whitespace(s);

      let s = utils::tag("is");
      let (s, _) = extract_whitespace(s);

      let (s, val) = Expr::new(s);

      (
         s,
         Self {
            name: name.to_string(),
            val,
         }
         )
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::expr::{Number, Op};

   #[test]
   fn parse_binding_def() {
      assert_eq!(
         BindingDef::new(
            "a is 10/2"),
            (
               "",
               BindingDef {
                  name: String::from("a"),
                  val: Expr {
                     lhs: Number(10),
                     rhs: Number(2),
                     op: Op::Div
                  }
               }
               )
         )
   }
}
