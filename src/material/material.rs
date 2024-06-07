use std::fmt::Display;

use crate::{
    ifc_type::IfcType,
    parser::{
        comma::Comma, label::Label, optional::OptionalParameter, p_space_or_comment_surrounded,
        IFCParse, IFCParser,
    },
};

/// IfcMaterial is a homogeneous or inhomogeneous substance that can be
/// used to form elements (physical products or their components).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcmaterial.htm
pub struct Material {
    /// Name of the material.
    pub material: OptionalParameter<Label>,

    /// Definition of the material in more descriptive terms than given by
    /// attributes Name or Category.
    pub description: OptionalParameter<Label>,

    /// Definition of the category (group or type) of material,
    /// in more general terms than given by attribute Name.
    pub category: OptionalParameter<Label>,
}

impl IFCParse for Material {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMATERIAL("),

                material: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
                _: Comma::parse(),
                category: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCMATERIAL({},{},{});",
            self.material, self.description, self.category,
        )
    }
}

impl IfcType for Material {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::Material;
    use crate::parser::IFCParse;

    #[test]
    fn material_layer_round_trip() {
        let example = "IFCMATERIAL('Masonry',$,$);";

        let parsed: Material = Material::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
