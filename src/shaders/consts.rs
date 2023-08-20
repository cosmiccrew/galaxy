use bevy::prelude::{HandleUntyped, Shader};
use bevy::reflect::TypeUuid;

pub const SHADER_TYPES: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 18223916570228862001);
