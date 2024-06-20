use std::collections::HashSet;

use glam::{DVec2, DVec3};

use crate::prelude::*;

pub struct SpaceParameter {
    pub coords: Vec<DVec2>,
    pub height: f64,
    pub placement: DVec3,
}

impl<'a> IfcStoreyBuilder<'a> {
    pub fn space(
        &mut self,
        space_type: TypedId<SpaceType>,
        name: &str,
        space_information: SpaceParameter,
    ) {
        let shape_repr_3d = ShapeRepresentation::new(self.sub_context, self.ifc).add_item(
            ExtrudedAreaSolid::new(
                ArbitraryClosedProfileDef::new(
                    ProfileType::Area,
                    IndexedPolyCurve::new(
                        PointList2D::new(space_information.coords.into_iter()),
                        self.ifc,
                    ),
                    self.ifc,
                ),
                Direction3D::from(DVec3::new(0.0, 0.0, 1.0)),
                space_information.height,
                self.ifc,
            ),
            self.ifc,
        );

        // TODO: add the footprint curve as an additional shaperepresentation to the space's
        // `ProductDefinitionShape.representations` vec
        let product_shape =
            ProductDefinitionShape::new().add_representation(shape_repr_3d, self.ifc);

        let position = Axis3D::new(Point3D::from(space_information.placement), self.ifc);
        let local_placement = LocalPlacement::new_relative(position, self.storey, self.ifc);

        let space = Space::new(name)
            .owner_history(self.owner_history, self.ifc)
            .object_placement(local_placement, self.ifc)
            .representation(product_shape, self.ifc);

        let space_id = self.ifc.data.insert_new(space);

        self.spaces.insert(space_id);
        self.space_type_to_space
            .get_mut(&space_type)
            .unwrap()
            .insert(space_id);
    }

    pub fn space_type(&mut self, name: &str, space_type: SpaceTypeEnum) -> TypedId<SpaceType> {
        let space_type = SpaceType::new(name, space_type)
            .owner_history(self.owner_history, self.ifc)
            .name(name);

        let space_type_id = self.ifc.data.insert_new(space_type);

        self.space_type_to_space
            .insert(space_type_id, HashSet::new());

        space_type_id
    }
}

#[cfg(test)]
mod test {
    use glam::{DVec2, DVec3};

    use crate::ifc_builder::spaces::SpaceParameter;
    use crate::prelude::*;

    use super::super::test::create_builder;

    #[test]
    fn builder_spaces() {
        let mut builder = create_builder();

        {
            let mut site_builder = builder.new_site("test", DVec3::ZERO);
            let mut building_builder = site_builder.new_building("test", DVec3::ZERO);
            let mut storey_builder = building_builder.new_storey("test", 0.0);

            let story_footprint = vec![
                DVec2::ZERO,
                DVec2::new(0.0, 4.0),
                DVec2::new(2.0, 6.0),
                DVec2::new(4.0, 4.0),
                DVec2::new(4.0, 0.0),
                DVec2::ZERO,
            ];

            let space_type = storey_builder.space_type("ExampleWallType", SpaceTypeEnum::Space);
            storey_builder.space(
                space_type,
                "ExampleSpaceDefault",
                SpaceParameter {
                    coords: story_footprint.clone(),
                    height: 4.0,
                    placement: DVec3::new(0.0, 0.0, 0.0),
                },
            );
        }

        let s = builder.build();
        let ifc = IFC::from_str(&s).unwrap();

        assert_eq!(s, ifc.to_string());
    }
}
