use crate::{
    id::Id,
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_list_of, p_space_or_comment_surrounded,
    },
};

use super::ProductDefinitionShape;

impl IFCParse for ProductDefinitionShape {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPRODUCTDEFINITIONSHAPE("),
                name: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                description: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                representations: p_list_of::<Id>(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[test]
fn parse_product_definition_shape_works() {
    use winnow::prelude::*;

    let data = "IFCPRODUCTDEFINITIONSHAPE($,$,(#256));";
    let parsed = ProductDefinitionShape::parse().parse(data).unwrap();
    assert_eq!(format!("{data}"), format!("{parsed}"));
}