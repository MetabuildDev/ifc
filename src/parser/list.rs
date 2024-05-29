use std::fmt::Display;

use itertools::Itertools;
use winnow::{
    combinator::{opt, preceded, repeat_till, terminated},
    Parser,
};

use super::{optional::IFCParse, p_space_or_comment_surrounded};

#[derive(Debug, Clone)]
pub struct IfcList<T>(pub Vec<T>);

impl<T: IFCParse> IFCParse for IfcList<T> {
    fn parse<'a>() -> impl super::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        let p_t_opt_comma = terminated(T::parse(), p_space_or_comment_surrounded(opt(",")));
        preceded(
            "(",
            repeat_till(.., p_t_opt_comma, ")").map(|(v, _): (Vec<_>, _)| v),
        )
        .map(Self)
    }
}

impl<T: Display> Display for IfcList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({items})",
            items = self.0.iter().map(ToString::to_string).join(",")
        )
    }
}