use crate::{Read, Write, error::Error};

macro_rules! define_id {
    ($($(#[$meta:meta])* $name:ident),*$(,)?) => {
        $(
            $(#[$meta])*
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name(pub i32);

            impl Read for $name {
                fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
                    Ok(Self(Read::read(input)?))
                }
            }

            impl Write for $name {
                fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
                    self.0.write(output)
                }
            }
        )*
    };
}

define_id!(
    /// The id of an object in the level.
    EntityId,
    /// The id of a tile type.
    TileId,
    /// The id of a prefab.
    PrefabId,
    /// The id of a pattern.
    PatternId,
    /// The id of a brush.
    BrushId,
    /// The id of a layer.
    LayerId,
    /// The id of a script.
    ScriptId,
    /// The id of a script variable.
    VariableId,
    /// The id of a script parameter.
    ParameterId,
);
