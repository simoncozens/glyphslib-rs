use std::collections::BTreeMap;

use crate::serde::{deserialize_commify, serialize_commify};
use openstep_plist::Plist;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FeatureClass {
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// A string containing space separated glyph names.
    pub code: String,
    /// The name of the class
    pub name: String,
    /// The class will not be exported
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// Notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CustomParameter {
    /// Property name of the custom parameter
    pub name: String,
    /// Value of the custom parameters
    pub value: Plist,
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FeaturePrefix {
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// A string containing feature code.
    code: String,
    /// The name of the prefix
    #[serde(alias = "tag")] // Of course some random Glyphs version did this
    name: String,
    /// The prefix will not be exported
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// Notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Feature {
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// A string containing feature code.
    code: String,
    /// The prefix will not be exported
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// List of stylistic set labels
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    labels: Vec<StylisticSetLabel>,
    /// Notes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The feature tag
    #[serde(alias = "name")]
    tag: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StylisticSetLabel {
    /// 'dflt' or three letter ISO language tag ("DEU")
    language: String,
    /// The name
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Color {
    ColorInt(u8),
    ColorTuple(Vec<u8>),
}

pub type Kerning = BTreeMap<String, BTreeMap<String, BTreeMap<String, f32>>>;

pub(crate) fn is_false(b: &bool) -> bool {
    !b
}
pub(crate) fn is_true(b: &bool) -> bool {
    *b
}
pub(crate) fn bool_true() -> bool {
    true
}

pub(crate) fn scale_unit() -> (f32, f32) {
    (1.0, 1.0)
}

pub(crate) fn is_scale_unit(scale: &(f32, f32)) -> bool {
    *scale == (1.0, 1.0)
}

// pub(crate) fn non_zero(b: &i32) -> bool {
//     *b != 0
// }

pub(crate) fn is_default<T>(v: &T) -> bool
where
    T: Default + PartialEq,
{
    *v == T::default()
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Copy)]
pub enum GuideAlignment {
    #[default]
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum NodeType {
    #[serde(rename = "l")]
    Line,
    #[serde(rename = "c")]
    Curve,
    #[serde(rename = "q")]
    QCurve,
    #[serde(rename = "o")]
    OffCurve,
    #[serde(rename = "ls")]
    LineSmooth,
    #[serde(rename = "cs")]
    CurveSmooth,
    #[serde(rename = "qs")]
    QCurveSmooth,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Version {
    #[serde(default, rename = "versionMajor")]
    pub major: i32,
    #[serde(default, rename = "versionMinor")]
    pub minor: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstanceFactors(
    #[serde(
        deserialize_with = "deserialize_commify",
        serialize_with = "serialize_commify",
        default
    )]
    pub Vec<f32>,
);
