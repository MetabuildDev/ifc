mod deserialize;
mod serialize;

use ifc_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{label::Label, optional::OptionalParameter},
    prelude::*,
};

use super::{indexed_poly_curve::Curve, rectangle_profile_def::ProfileDef};

/// The closed profile IfcArbitraryClosedProfileDef defines an arbitrary
/// two-dimensional profile for the use within the swept surface geometry,
/// the swept area solid or a sectioned spine. It is given by an outer
/// boundary from which the surface or solid can be constructed.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcprofileresource/lexical/ifcarbitraryclosedprofiledef.htm
#[derive(IfcVerify)]
pub struct ArbitraryClosedProfileDef<C: Curve> {
    /// Defines the type of geometry into which this profile definition shall be resolved, either a
    /// curve or a surface area. In case of curve the profile should be referenced by a swept
    /// surface, in case of area the profile should be referenced by a swept area solid.
    pub profile_type: ProfileType,
    /// Human-readable name of the profile, for example according to a standard profile table. As
    /// noted above, machine-readable standardized profile designations should be provided in
    /// IfcExternalReference.ItemReference.
    pub profile_name: OptionalParameter<Label>,
    /// `IfcCurve` Bounded curve, defining the outer boundaries of the arbitrary profile.
    pub outer_curve: TypedId<C>,
}

impl<C: Curve> ArbitraryClosedProfileDef<C> {
    pub fn new(profile_type: ProfileType, outer_curve: impl Into<IdOr<C>>, ifc: &mut IFC) -> Self {
        Self {
            profile_type,
            profile_name: OptionalParameter::omitted(),
            outer_curve: outer_curve.into().or_insert(ifc),
        }
    }

    pub fn profile_name(mut self, name: impl Into<Label>) -> Self {
        self.profile_name = name.into().into();
        self
    }
}

impl<C: Curve> IfcType for ArbitraryClosedProfileDef<C> {}
impl<C: Curve> ProfileDef for ArbitraryClosedProfileDef<C> {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::geometry::arbitrary_closed_profile_def::ArbitraryClosedProfileDef;
    use crate::parser::IFCParse;
    use crate::prelude::IndexedPolyCurve;

    #[test]
    fn arbitrary_closed_profile_def_round_trip() {
        let example = "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,$,#25);";

        let profile_def = ArbitraryClosedProfileDef::<IndexedPolyCurve>::parse()
            .parse(example)
            .unwrap();
        let str_profile_def = format!("{profile_def}");

        assert_eq!(example, str_profile_def);
    }
}
