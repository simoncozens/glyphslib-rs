use std::collections::BTreeMap;

use crate::serde::{deserialize_commify, is_default, serialize_commify};
use openstep_plist::Plist;
use serde::{Deserialize, Serialize};

/// The OpenType layout classes of the font (`GSClass`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FeatureClass {
    /// Whether the code of the class is generated automatically.
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// The code of the class.
    ///
    /// Note that this code may not just be a whitespace-separated list of glyph names but may also contain comments and other feature code constructs. Examples: "A B C", "noon-ar noon-ar.fina noon-ar.medi noon-ar.init # noon-ar glyphs".
    pub code: String,
    /// The name of the class. The leading at sign (`@`) is not included. Examples: `"Uppercase"`, `"CombiningTopAccents"`.
    pub name: String,
    /// Whether the class is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// A string serving as a description or comment about the class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Custom parameter (`GSCustomParameter`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CustomParameter {
    /// The name of the custom parameter.
    pub name: String,
    /// The value of the custom parameter.
    pub value: Plist,
    /// Whether the custom parameter is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
}

/// Feature prefix (`GSFeaturePrefix`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FeaturePrefix {
    /// Whether the code of the feature prefix is generated automatically.
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// The code of the feature prefix. Example: `"languagesystem DFLT dflt;"`.
    pub code: String,
    /// The name of the feature prefix. Example: `"Languagesystems"`.
    #[serde(alias = "tag")] // Of course some random Glyphs version did this
    pub name: String,
    /// Whether the feature prefix is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// A string serving as a description or comment about the feature prefix.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Feature (`GSFeature`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Feature {
    /// Whether the code of the feature is generated automatically.
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// The code of the feature. Example: `"sub a by a.alt;"`.
    pub code: String,
    /// Whether the feature is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// The labels of the feature.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<StylisticSetLabel>,
    /// A string serving as a description or comment about the feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The four-letter tag of the feature. Example: `"calt"`.
    #[serde(alias = "name")]
    pub tag: String,
}

/// Stylistic set label (`GSInfoValue`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StylisticSetLabel {
    /// The language tag of the string value. The tag is based on the OpenType Language System Tags but omits trailing whitespace. Examples: `"dflt"`, `"DEU"`.
    language: String,
    /// The localized string value.
    value: String,
}

/// Color representation
///
/// Can be:
/// - An RGB color with an alpha channel in the sRGB IEC61966-2.1 color space (4 components)
/// - A gray color with an alpha channel in a perceptual generic gray color space with γ = 2.2 (2 components)
/// - A CMYK color with an alpha channel, device-dependent color space (5 components)
/// - An integer index of the color label
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Color {
    /// The index of the color label.
    ColorInt(u8),
    /// Color tuple (RGB, Gray, or CMYK with alpha channel)
    ColorTuple(Vec<u8>),
}

/// Kerning definition mapping master IDs to kerning definitions, which map glyph names or class names to kerning partners.
pub type Kerning = BTreeMap<String, BTreeMap<String, BTreeMap<String, f32>>>;

/// Guide alignment (`GSElementOrientation`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Copy)]
pub enum Orientation {
    /// Left alignment
    #[default]
    #[serde(rename = "left")]
    Left,
    /// Center alignment
    #[serde(rename = "center")]
    Center,
    /// Right alignment
    #[serde(rename = "right")]
    Right,
}

/// Node type for path nodes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum NodeType {
    /// Line node
    #[serde(rename = "l")]
    Line,
    /// Curve node
    #[serde(rename = "c")]
    Curve,
    /// QCurve node
    #[serde(rename = "q")]
    QCurve,
    /// Off-curve node
    #[serde(rename = "o")]
    OffCurve,
    /// Line smooth node
    #[serde(rename = "ls")]
    LineSmooth,
    /// Curve smooth node
    #[serde(rename = "cs")]
    CurveSmooth,
    /// QCurve smooth node
    #[serde(rename = "qs")]
    QCurveSmooth,
}

/// Version information
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Version {
    /// The major version number of the font.
    #[serde(default, rename = "versionMajor")]
    pub major: i32,
    /// The minor version number of the font.
    #[serde(default, rename = "versionMinor")]
    pub minor: i32,
}

/// Instance interpolation factors
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstanceFactors(
    #[serde(
        deserialize_with = "deserialize_commify",
        serialize_with = "serialize_commify",
        default
    )]
    pub Vec<f32>,
);
