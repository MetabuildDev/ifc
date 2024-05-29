use winnow::combinator::{preceded, repeat_till};

use crate::{
    id::{Id, IdOr},
    parser::{optional::IFCParse, p_space_or_comment_surrounded},
};

use super::ShapeRepresentation;

impl IFCParse for ShapeRepresentation {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSHAPEREPRESENTATION("),
                context_of_items: Id::parse(),
                _: p_space_or_comment_surrounded(","),
                representation_identifier: IdOr::parse(),
                _: p_space_or_comment_surrounded(","),
                representation_type: IdOr::parse(),
                _: p_space_or_comment_surrounded(","),
                items: preceded("(", repeat_till(.., Id::parse(), ")")).map(|(v, _): (Vec<_>, _)| v),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_shape_representation_works() {
    use winnow::prelude::*;

    let data = "IFCSHAPEREPRESENTATION(#107,'Body','MappedRepresentation',(#2921786));";
    let parsed = ShapeRepresentation::parse().parse(data).unwrap();
    assert_eq!(format!("{data}"), format!("{parsed}"));
}