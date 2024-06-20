use std::ops::DerefMut;
use std::{fmt::Display, ops::Deref};

use ifc_verify_derive::IfcVerify;

use crate::{
    ifc_type::IfcVerify,
    parser::{comma::Comma, label::Label, optional::OptionalParameter, IFCParse, IFCParser},
    prelude::*,
};

use super::root::Root;

/// An IfcObject is the generalization of any semantically treated
/// thing or process. Objects are things as they appear - i.e. occurrences.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcobject.htm
#[derive(IfcVerify)]
pub struct Object {
    #[inherited]
    root: Root,

    /// The type denotes a particular type that indicates the object further.
    /// The use has to be established at the level of instantiable subtypes.
    /// In particular it holds the user defined type, if the enumeration
    /// of the attribute PredefinedType is set to USERDEFINED.
    pub object_type: OptionalParameter<Label>,
}

impl Object {
    pub fn new(root: Root) -> Self {
        Self {
            root,
            object_type: OptionalParameter::omitted(),
        }
    }
}

pub trait ObjectBuilder: Sized {
    fn object_mut(&mut self) -> &mut Object;

    fn object_type(mut self, object_type: impl Into<Label>) -> Self {
        self.object_mut().object_type = object_type.into().into();
        self
    }
}

impl Deref for Object {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
    }
}

impl IFCParse for Object {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: Comma::parse(),
                object_type: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.root, self.object_type)
    }
}
