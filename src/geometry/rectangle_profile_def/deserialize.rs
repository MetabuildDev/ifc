use winnow::ascii::float;

use crate::geometry::profile_type::ProfileType;
use crate::parser::optional::{IFCParse, OptionalParameter};
use crate::parser::place_holder::PlaceHolder;
use crate::parser::*;

use super::RectangleProfileDef;

impl IFCParse for RectangleProfileDef {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment(), "IFCRECTANGLEPROFILEDEF(", p_space_or_comment()),
                profile_type: ProfileType::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                _: PlaceHolder::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                position: OptionalParameter::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                x_dim: float,
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                y_dim: float,
                _: (p_space_or_comment(), ");", p_space_or_comment()),
            }
        }
    }
}
