use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt;

use crate::serde::{deserialize_commify, is_default, serialize_commify};
use openstep_plist::Plist;
use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeTuple as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    /// Whether the class is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// The name of the class. The leading at sign (`@`) is not included. Examples: `"Uppercase"`, `"CombiningTopAccents"`.
    pub name: String,
    /// A string serving as a description or comment about the class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Custom parameter (`GSCustomParameter`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct CustomParameter {
    /// Whether the custom parameter is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// The name of the custom parameter.
    pub name: String,
    /// The value of the custom parameter.
    pub value: Plist,
}

/// Feature prefix (`GSFeaturePrefix`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FeaturePrefix {
    /// Whether the code of the feature prefix is generated automatically.
    #[serde(default, skip_serializing_if = "is_default")]
    pub automatic: bool,
    /// The code of the feature prefix. Example: `"languagesystem DFLT dflt;"`.
    pub code: String,
    /// Whether the feature prefix is disabled.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    /// The name of the feature prefix. Example: `"Languagesystems"`.
    #[serde(alias = "tag")] // Of course some random Glyphs version did this
    pub name: String,
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
    pub language: String,
    /// The localized string value.
    pub value: String,
}

/// File format versions that change how [`Color`] is encoded: v3 uses `0...255`
/// integers, v4 uses normalized `0...1` floats and adds palette references.
thread_local! {
    static FORMAT_VERSION: Cell<Option<u8>> = const { Cell::new(None) };
}

/// Fallback used when [`current_format_version`] is called outside a scoped
/// load/save (shouldn't happen in practice). Matches `FormatVersion::default()`.
const DEFAULT_FORMAT_VERSION: u8 = 3;

/// Sets the format version for the current thread and restores the previous value
/// on drop, so the setting is cleared even if the load/save panics.
///
/// ```ignore
/// let _guard = with_format_version(4);
/// openstep_plist::ser::to_string(glyphs3)
/// ```
#[must_use = "the guard restores the previous version when dropped"]
pub fn with_format_version(version: u8) -> FormatVersionGuard {
    let previous = FORMAT_VERSION.with(|cell| cell.replace(Some(version)));
    FormatVersionGuard(previous)
}

/// Restores the previous format version when dropped. See [`with_format_version`].
pub struct FormatVersionGuard(Option<u8>);

impl Drop for FormatVersionGuard {
    fn drop(&mut self) {
        FORMAT_VERSION.with(|cell| cell.set(self.0));
    }
}

fn current_format_version() -> u8 {
    FORMAT_VERSION
        .with(Cell::get)
        .unwrap_or(DEFAULT_FORMAT_VERSION)
}

/// Color representation
///
/// Can be:
/// - An RGB color with an alpha channel in the sRGB IEC61966-2.1 color space (4 components)
/// - A gray color with an alpha channel in a perceptual generic gray color space with γ = 2.2 (2 components)
/// - A CMYK color with an alpha channel, device-dependent color space (5 components)
/// - An integer index of the color label
/// - An index and alpha channel into the font's Color Palettes custom parameter (V4)
///
/// Tuples are stored normalized to `0...1`. They are scaled to `0...255`
/// integers when the active format version is below 4.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// The index of the color label.
    ColorInt(u8),
    /// Color tuple (RGB, Gray, or CMYK with alpha channel)
    ColorTuple(Vec<f64>),
    /// A color from a color palette
    ColorPaletteIndex {
        /// The index of the color in the palette.
        index: u16,
        /// The alpha channel of the color.
        alpha: f64,
    },
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Color::ColorInt(index) => serializer.serialize_u8(*index),
            Color::ColorPaletteIndex { index, alpha } => {
                let mut tuple = serializer.serialize_tuple(3)?;
                tuple.serialize_element("p")?;
                tuple.serialize_element(index)?;
                tuple.serialize_element(alpha)?;
                tuple.end()
            }
            Color::ColorTuple(components) => {
                let mut tuple = serializer.serialize_tuple(components.len())?;
                if current_format_version() >= 4 {
                    for component in components {
                        tuple.serialize_element(component)?;
                    }
                } else {
                    for component in components {
                        let scaled = (component * 255.0).round() as u8;
                        tuple.serialize_element(&scaled)?;
                    }
                }
                tuple.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("a color label index, a list of color components, or a palette reference")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Color::ColorInt(value as u8))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Color::ColorInt(value as u8))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        match seq.next_element::<ColorElement>()? {
            // Version 4 palette reference: ("p", index, alpha).
            Some(ColorElement::Marker(marker)) => {
                if marker != "p" {
                    return Err(de::Error::unknown_variant(&marker, &["p"]));
                }
                let index = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let alpha = seq.next_element()?.unwrap_or(1.0);
                Ok(Color::ColorPaletteIndex { index, alpha })
            }
            // A list of color components.
            Some(ColorElement::Number(first)) => {
                let mut components = vec![first];
                while let Some(component) = seq.next_element()? {
                    components.push(component);
                }
                if current_format_version() < 4 {
                    // Version 3 components are `0...255`; normalize to `0...1`.
                    for component in &mut components {
                        *component /= 255.0;
                    }
                }
                Ok(Color::ColorTuple(components))
            }
            None => Ok(Color::ColorTuple(Vec::new())),
        }
    }
}

/// First element of a `Color` array: either the `"p"` palette marker or a
/// numeric color component.
#[derive(Deserialize)]
#[serde(untagged)]
enum ColorElement {
    Marker(String),
    Number(f64),
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

/// Smart component property setting (`GSPartProperty`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SmartComponentSetting {
    /// The lower end of the value range of the property.
    #[serde(default, rename = "bottomValue")]
    pub bottom_value: i32,
    /// The upper end of the value range of the property.
    #[serde(default, rename = "topValue")]
    pub top_value: i32,
    /// The name of the property.
    pub name: String,
}
