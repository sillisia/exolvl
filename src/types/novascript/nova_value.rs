use crate::{
    Read, Write,
    error::Error,
    types::{color::Color, dynamic_type::DynamicType, vec2::Vec2},
};
use ordered_float::OrderedFloat;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NovaValue {
    pub dynamic_type: DynamicType,

    pub(crate) inner: NovaValueInner,
}

impl NovaValue {
    #[must_use]
    pub fn new_bool(ty: DynamicType, value: bool) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                bool_value: value,
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_int(ty: DynamicType, value: i32) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                int_value: value,
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_float(ty: DynamicType, value: f32) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                float_value: OrderedFloat(value),
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_string(ty: DynamicType, value: String) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                string_value: Some(value),
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_color(ty: DynamicType, value: Color) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                color_value: value,
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_vector(ty: DynamicType, value: Vec2) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                vector_value: value,
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_int_list(ty: DynamicType, value: Vec<i32>) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                int_list_value: Some(value),
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn new_sub_values(ty: DynamicType, value: Vec<Self>) -> Self {
        Self {
            dynamic_type: ty,
            inner: NovaValueInner {
                sub_values: Some(value),
                ..Default::default()
            },
        }
    }
}

impl Read for NovaValue {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let dynamic_type = Read::read(input)?;
        let bool_value = Read::read(input)?;
        let int_value = Read::read(input)?;
        let float_value = Read::read(input)?;
        let string_value = Read::read(input)?;
        let color_value = Read::read(input)?;
        let vector_value = Read::read(input)?;
        let mut unknown5 = [0u8; 1];
        input.read_exact(&mut unknown5)?;
        let int_list_value = Read::read(input)?;
        let unknown6 = Read::read(input)?;
        let sub_values = Read::read(input)?;

        Ok(Self {
            dynamic_type,
            inner: NovaValueInner {
                bool_value,
                int_value,
                float_value,
                string_value,
                color_value,
                vector_value,
                unknown5,
                int_list_value,
                unknown6,
                sub_values,
            },
        })
    }
}

impl Write for NovaValue {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.dynamic_type.write(output)?;
        self.inner.bool_value.write(output)?;
        self.inner.int_value.write(output)?;
        self.inner.float_value.write(output)?;
        self.inner.string_value.write(output)?;
        self.inner.color_value.write(output)?;
        self.inner.vector_value.write(output)?;
        output.write_all(&self.inner.unknown5)?;
        self.inner.int_list_value.write(output)?;
        self.inner.unknown6.write(output)?;
        self.inner.sub_values.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NovaValueInner {
    pub bool_value: bool,
    pub int_value: i32,
    pub float_value: OrderedFloat<f32>,
    pub string_value: Option<String>,
    pub color_value: Color,
    pub vector_value: Vec2,
    /// Unknown data, new in serialization version 23 (sits right before `int_list_value`).
    pub unknown5: [u8; 1],
    pub int_list_value: Option<Vec<i32>>,
    /// Unknown data, new in serialization version 23 (sits between `int_list_value` and `sub_values`).
    pub unknown6: [u8; 4],
    pub sub_values: Option<Vec<NovaValue>>,
}
