use crate::{
  model::{ ScalarNumericTypeModel, TypeModel, VecDims },
  syntax::{
    name::NamePath,
    types::TypeName,
  },
};

/**
 * A partial instantiation of a type reference.
 */
#[derive(Debug, Clone)]
pub enum TypeRefPartial<'a> {
    Model(TypeModel),
    Path(NamePath<'a>),
}
impl<'a> TypeRefPartial<'a> {
  pub(crate) fn from_type_name(s: &TypeName<'a>)
    -> TypeRefPartial<'a>
  {
    let name = s.name.parts.last().expect("Empty type name").contents;
    match name {
      "i32" => TypeRefPartial::Model(TypeModel::new_i32()),
      "u32" => TypeRefPartial::Model(TypeModel::new_u32()),
      "f32" => TypeRefPartial::Model(TypeModel::new_f32()),
      "bool" => TypeRefPartial::Model(TypeModel::new_bool()),
      "void" => TypeRefPartial::Model(TypeModel::new_void()),
      "vec2xi32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::I32, VecDims::Vec2)
      ),
      "vec2xu32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::U32, VecDims::Vec2)
      ),
      "vec2xf32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::F32, VecDims::Vec2)
      ),
      "vec3xi32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::I32, VecDims::Vec3)
      ),
      "vec3xu32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::U32, VecDims::Vec3)
      ),
      "vec3xf32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::F32, VecDims::Vec3)
      ),
      "vec4xi32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::I32, VecDims::Vec4)
      ),
      "vec4xu32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::U32, VecDims::Vec4)
      ),
      "vec4xf32" => TypeRefPartial::Model(
          TypeModel::new_vector(ScalarNumericTypeModel::F32, VecDims::Vec4)
      ),
      _ => TypeRefPartial::Path(s.name.clone()),
    }
  }
}
