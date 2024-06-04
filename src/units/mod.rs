pub mod angle;
pub mod area;

pub mod name;
pub mod prefix;

use std::fmt::Display as StdDisplay;
use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::optional::IFCParse;
use crate::parser::*;

pub struct Units;

impl Units {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn StdDisplay>> {
        alt((angle::AngleUnit::parse_any(), area::AreaUnit::parse_any()))
    }
}

impl StdDisplay for Units {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

// TODO: there are a lot more (mostly imperial units)
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum ConversionUnit {
    #[strum(to_string = "'DEGREE'")]
    Degree,

    #[strum(to_string = "'LITRE'")]
    Litre,

    #[strum(to_string = "'MINUTE'")]
    Minute,

    #[strum(to_string = "'HOUR'")]
    Hour,

    #[strum(to_string = "'DAY'")]
    Day,
}

impl IFCParse for ConversionUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid ConversionUnit")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
