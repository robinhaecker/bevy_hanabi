//! Effect graph and language definition.
//!
//! This module contains the elements used to build a effect graph, a fully
//! customizable description of a visual effect.
//!
//! Currently effect graphs are not yet available; only some preview elements
//! exist. So this module is a bit empty and of little interest.

use std::fmt::Debug;

use bevy::{
    math::{Vec2, Vec3, Vec4},
    reflect::{FromReflect, Reflect},
};
use serde::{Deserialize, Serialize};

use crate::{ToWgslString, ValueType};

/// Variant storage for a simple value.
#[derive(Debug, Clone, Copy, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Value {
    /// Single `f32` value.
    Float(f32),
    /// Vector of two `f32` values (`vec2<f32>`).
    Float2(Vec2),
    /// Vector of three `f32` values (`vec3<f32>`).
    Float3(Vec3),
    /// Vector of four `f32` values (`vec4<f32>`).
    Float4(Vec4),
    /// Single `u32` value.
    Uint(u32),
}

impl Value {
    /// Get the value as a binary blob ready for GPU upload.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Value::Float(f) => bytemuck::cast_slice::<f32, u8>(std::slice::from_ref(f)),
            Value::Float2(v) => bytemuck::cast_slice::<Vec2, u8>(std::slice::from_ref(v)),
            Value::Float3(v) => bytemuck::cast_slice::<Vec3, u8>(std::slice::from_ref(v)),
            Value::Float4(v) => bytemuck::cast_slice::<Vec4, u8>(std::slice::from_ref(v)),
            Value::Uint(u) => bytemuck::cast_slice::<u32, u8>(std::slice::from_ref(u)),
        }
    }
}

impl Value {
    /// Type of the value.
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Float(_) => ValueType::Float,
            Value::Float2(_) => ValueType::Float2,
            Value::Float3(_) => ValueType::Float3,
            Value::Float4(_) => ValueType::Float4,
            Value::Uint(_) => ValueType::Uint,
        }
    }
}

impl ToWgslString for Value {
    fn to_wgsl_string(&self) -> String {
        match self {
            Value::Float(f) => f.to_wgsl_string(),
            Value::Float2(v2) => v2.to_wgsl_string(),
            Value::Float3(v3) => v3.to_wgsl_string(),
            Value::Float4(v4) => v4.to_wgsl_string(),
            Value::Uint(u) => u.to_wgsl_string(),
        }
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Self::Float(f)
    }
}

impl From<Vec2> for Value {
    fn from(v: Vec2) -> Self {
        Self::Float2(v)
    }
}

impl From<Vec3> for Value {
    fn from(v: Vec3) -> Self {
        Self::Float3(v)
    }
}

impl From<Vec4> for Value {
    fn from(v: Vec4) -> Self {
        Self::Float4(v)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Self {
        Self::Uint(u)
    }
}