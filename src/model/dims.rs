use crate::syntax::declaration::EntrypointDeclDims;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EntrypointDims {
  D1 = 1,
  D2 = 2,
  D3 = 3,
}
impl EntrypointDims {
  pub fn from_decl_dims(decl_dims: EntrypointDeclDims) -> Self {
    match decl_dims {
      EntrypointDeclDims::D1 => Self::D1,
      EntrypointDeclDims::D2 => Self::D2,
      EntrypointDeclDims::D3 => Self::D3,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VecDims {
  Vec2 = 2,
  Vec3 = 3,
  Vec4 = 4,
}
