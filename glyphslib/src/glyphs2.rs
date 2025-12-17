use std::collections::BTreeMap;

use crate::{
    common::InstanceFactors,
    serde::{
        anything_to_bool, bool_true, deserialize_comma_hexstring, deserialize_commify, is_default,
        is_false, is_scale_unit, is_true, scale_unit, serialize_comma_hexstring, serialize_commify,
    },
};

use openstep_plist::Dictionary;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    common::{
        Color, CustomParameter, Feature, FeatureClass, FeaturePrefix, Kerning, NodeType,
        Orientation, Version,
    },
    serde::{is_one_hundred, one_hundred},
};

/// Glyphs file format version 2 document
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Glyphs2 {
    /// The build number of Glyphs used to save the file. Example: `"3210"`.
    #[serde(
        rename = ".appVersion",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub app_version: String,
    /// The strings of the Edit View tabs. Omitted when the `Write DisplayStrings` custom parameter is set to false.
    #[serde(
        rename = "DisplayStrings",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub display_strings: Vec<String>,
    /// The OpenType layout classes of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<FeatureClass>,
    /// The copyright notice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    /// The custom parameters of the font.
    #[serde(
        default,
        rename = "customParameters",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    /// The moment in time that is used as the creation date of exported font files including date, time, and timezone. Example: `"2024-07-17 03:14:15 +0000"`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub date: String,
    /// The name of the designer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub designer: Option<String>,
    /// The URL to the designer.
    #[serde(
        default,
        rename = "designerURL",
        skip_serializing_if = "Option::is_none"
    )]
    pub designer_url: Option<String>,
    /// Whether automatic alignment of components is disabled.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "disablesAutomaticAlignment"
    )]
    pub disables_automatic_alignment: bool,
    /// Whether to use production names instead of nice names.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "disablesNiceNames"
    )]
    pub disables_nice_names: bool,
    /// The font family name.
    #[serde(rename = "familyName")]
    pub family_name: String,
    /// The OpenType layout feature prefixes of the font.
    #[serde(
        default,
        rename = "featurePrefixes",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub feature_prefixes: Vec<FeaturePrefix>,
    /// The OpenType layout features of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<Feature>,
    /// The masters of the font.
    #[serde(rename = "fontMaster", skip_serializing_if = "Vec::is_empty", default)]
    pub masters: Vec<Master>,
    /// The glyphs of the font. The order is used on export unless the `glyphOrder` custom parameter is set.
    #[serde(default)]
    pub glyphs: Vec<Glyph>,
    /// The main grid length.
    #[serde(rename = "gridLength", skip_serializing_if = "Option::is_none")]
    pub grid_length: Option<i32>,
    /// The grid sub-division size.
    #[serde(rename = "gridSubDivision", skip_serializing_if = "Option::is_none")]
    pub grid_sub_division: Option<i32>,

    /// The instances of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<Instance>,
    /// Whether to keep alternates glyphs together in Font View.
    #[serde(
        rename = "keepAlternatesTogether",
        default,
        skip_serializing_if = "is_default"
    )]
    pub keep_alternates_together: bool,
    /// The kerning of the font.
    #[serde(default, skip_serializing_if = "Kerning::is_empty")]
    pub kerning: Kerning,
    /// The vertical kerning of the font.
    #[serde(
        rename = "vertKerning",
        default,
        skip_serializing_if = "Kerning::is_empty"
    )]
    pub kerning_vertical: Kerning,
    /// The standard keyboard increment.
    #[serde(rename = "keyboardIncrement", skip_serializing_if = "Option::is_none")]
    pub keyboard_increment: Option<f32>,
    /// The name of the manufacturer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The URL to the manufacturer.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "manufacturerURL"
    )]
    pub manufacturer_url: Option<String>,
    /// The number of coordinate units on the em square.
    #[serde(rename = "unitsPerEm")]
    pub units_per_em: i32,
    /// Custom data associated with the font.
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    /// Version information.
    #[serde(flatten, default, skip_serializing_if = "is_default")]
    pub version: Version,
}

/// Font master (`GSFontMaster`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Master {
    /// The alignment zones of the master.
    #[serde(
        rename = "alignmentZones",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub alignment_zones: Vec<AlignmentZone>,
    /// The ascender metric of the master.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ascender: Option<f32>,
    /// The cap height metric of the master.
    #[serde(rename = "capHeight", default, skip_serializing_if = "Option::is_none")]
    pub cap_height: Option<f32>,
    /// The custom name of the master.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    /// The location on the custom axis of the master.
    #[serde(default, skip_serializing_if = "is_default", rename = "customValue")]
    pub custom_value: i32,
    /// The location on the first custom axis of the master.
    #[serde(default, skip_serializing_if = "is_default", rename = "customValue1")]
    pub custom_value_1: i32,
    /// The location on the second custom axis of the master.
    #[serde(default, skip_serializing_if = "is_default", rename = "customValue2")]
    pub custom_value_2: i32,
    /// The location on the third custom axis of the master.
    #[serde(default, skip_serializing_if = "is_default", rename = "customValue3")]
    pub custom_value_3: i32,
    /// The custom parameters of the master.
    #[serde(
        rename = "customParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    /// The descender metric of the master.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descender: Option<f32>,
    /// The horizontal stems of the master.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "horizontalStems"
    )]
    pub horizontal_stems: Vec<i32>,
    /// The name of the icon that represents the master. Generally omitted when equal to `Regular`, or equal to the default icon name of the master.
    #[serde(rename = "iconName", default, skip_serializing_if = "String::is_empty")]
    pub icon_name: String,
    /// The unique identifier of the master.
    pub id: String,
    /// The italic angle of the master in degrees clockwise.
    #[serde(default, skip_serializing_if = "is_default", rename = "italicAngle")]
    pub italic_angle: f32,
    /// The name of the master.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Custom data associated with the master.
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    /// The vertical stems of the master.
    #[serde(
        rename = "verticalStems",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vertical_stems: Vec<i32>,
    /// Whether the master is visible in the preview.
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub visible: bool,
    /// The weight name of the master.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub weight: String,
    /// The location on the weight axis of the master.
    #[serde(
        default = "one_hundred",
        skip_serializing_if = "is_one_hundred",
        rename = "weightValue"
    )]
    pub weight_value: i32,
    /// The width name of the master.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub width: String,
    /// The location on the width axis of the master.
    #[serde(
        default = "one_hundred",
        skip_serializing_if = "is_one_hundred",
        rename = "widthValue"
    )]
    pub width_value: i32,
    /// The x-height metric of the master.
    #[serde(default, rename = "xHeight", skip_serializing_if = "Option::is_none")]
    pub x_height: Option<f32>,
}

/// Alignment zone with position and overshoot (e.g. "{800, 15}")
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AlignmentZone {
    /// The position of the zone.
    pub position: f32,
    /// The overshoot of the zone.
    pub overshoot: f32,
}

/// Font instance (`GSInstance`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Instance {
    /// The custom parameters of the instance.
    #[serde(
        rename = "customParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    /// Whether the instance is active for export.
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub exports: bool,
    /// The third interpolation coefficient of the instance for each master.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "interpolationCustom"
    )]
    pub custom_value: f32,
    /// The fourth interpolation coefficient of the instance for each master.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "interpolationCustom1"
    )]
    pub custom_value_1: f32,
    /// The fifth interpolation coefficient of the instance for each master.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "interpolationCustom2"
    )]
    pub custom_value_2: f32,
    /// The sixth interpolation coefficient of the instance for each master.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "interpolationCustom3"
    )]
    pub custom_value_3: f32,
    /// The first interpolation coefficient of the instance for each master. The order of coefficients follows the order of masters.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "interpolationWeight"
    )]
    pub weight_value: f32,
    /// The second interpolation coefficient of the instance for each master.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "interpolationWidth"
    )]
    pub width_value: f32,
    /// The interpolation weight for each master. Keys are master IDs, values are the factors for that master.
    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        rename = "instanceInterpolations"
    )]
    pub instance_interpolations: BTreeMap<String, InstanceFactors>,
    /// Whether the instance is a bold. For style linking. Always set to 1, otherwise omit the key.
    #[serde(default, rename = "isBold", skip_serializing_if = "is_false")]
    pub is_bold: bool,
    /// Whether the instance is an italic. For style linking. Always set to 1, otherwise omit the key.
    #[serde(default, rename = "isItalic", skip_serializing_if = "is_false")]
    pub is_italic: bool,
    /// The family name that is used when exporting the instance as a static font.
    #[serde(default, rename = "isRegular")]
    pub link_style: Option<String>,
    /// Whether to use the instanceInterpolations instead of calculating from axis values. Always set to 1, otherwise omit the key.
    #[serde(
        default,
        rename = "manualInterpolation",
        skip_serializing_if = "is_false"
    )]
    pub manual_interpolation: bool,
    /// The style name of the instance.
    pub name: String,
    /// Custom data associated with the instance.
    #[serde(default, rename = "userData", skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    /// The weight class of the instance.
    #[serde(
        default,
        rename = "weightClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub weight_class: Option<String>,
    /// The width class of the instance.
    #[serde(
        default,
        rename = "widthClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub width_class: Option<String>,
}

/// Glyph (`GSGlyph`)
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Glyph {
    /// The bottom kerning group of the glyph.
    #[serde(rename = "bottomKerningGroup", skip_serializing_if = "is_default")]
    pub kern_bottom: Option<String>,
    /// The bottom metrics key of the glyph.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "bottomMetricsKey"
    )]
    pub metric_bottom: Option<String>,

    /// The name of the glyph.
    #[serde(rename = "glyphname")]
    pub name: String,
    /// An alternate name for the glyph. Generally omitted when equal to the glyph name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production: Option<String>,
    /// The script of the glyph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    /// The category of the glyph. Example: `Letter`, `Punctuation`, `Mark`, `Number`, `Separator`, `Symbol`, `Other`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The OpenType glyph color palette index. Between 0 and 65534.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Whether to export the glyph.
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub export: bool,
    /// The left kerning group of the glyph.
    #[serde(rename = "leftKerningGroup", skip_serializing_if = "is_default")]
    pub kern_left: Option<String>,
    /// The right kerning group of the glyph.
    #[serde(rename = "rightKerningGroup", skip_serializing_if = "is_default")]
    pub kern_right: Option<String>,
    /// The top kerning group of the glyph.
    #[serde(rename = "topKerningGroup", skip_serializing_if = "is_default")]
    pub kern_top: Option<String>,
    /// The date when the glyph was last modified, including date, time, and timezone. Example: `\"2017-10-31 07:41:24 +0000\"`.
    #[serde(rename = "lastChange", skip_serializing_if = "is_default")]
    pub last_change: Option<String>,
    /// The layers of the glyph.
    pub layers: Vec<Layer>,

    /// The Unicode values of the glyph as a comma-separated hexadecimal string. Example: `\"00660069\"` means the glyph represents Unicode codepoints U+0066 (f) and U+0069 (i).
    #[serde(
        default,
        deserialize_with = "deserialize_comma_hexstring",
        serialize_with = "serialize_comma_hexstring",
        skip_serializing_if = "Vec::is_empty",
        alias = "unicodes"
    )]
    pub unicode: Vec<u32>,
}

/// Layer (`GSLayer`)
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Layer {
    /// The anchors of the layer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anchors: Vec<Anchor>,
    /// The annotations of the layer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Dictionary>,

    /// The unique identifier of the associated master. Not present if it equals layerId, i.e. if the layer is in use as master.
    #[serde(
        rename = "associatedMasterId",
        default,
        skip_serializing_if = "is_default"
    )]
    pub associated_master_id: Option<String>,
    /// The background layer of the layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Box<Layer>>,
    /// The background image of the layer.
    #[serde(
        rename = "backgroundImage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub background_image: Option<BackgroundImage>,
    /// The components of the layer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
    /// The guides of the layer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guidelines: Vec<Guide>,
    /// The hints of the layer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hints: Vec<Hint>,
    /// The unique identifier of the layer.
    #[serde(rename = "layerId", default, skip_serializing_if = "String::is_empty")]
    pub layer_id: String,
    /// The left metrics key of the layer.
    #[serde(rename = "leftMetricsKey", default, skip_serializing_if = "is_default")]
    pub metric_left: Option<String>,
    /// The right metrics key of the layer.
    #[serde(
        rename = "rightMetricsKey",
        default,
        skip_serializing_if = "is_default"
    )]
    pub metric_right: Option<String>,
    /// The width metrics key of the layer.
    #[serde(
        rename = "widthMetricsKey",
        default,
        skip_serializing_if = "is_default"
    )]
    pub metric_width: Option<String>,
    /// The name of the layer. Only stored for non-master layers.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: Option<String>,
    /// The paths of the layer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Path>,
    /// Custom data associated with the layer.
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    /// The vertical width of the layer. Only stored if other than the default (ascender+descender).
    #[serde(rename = "vertWidth", default, skip_serializing_if = "is_default")]
    pub vert_width: Option<f32>,
    /// The width of the layer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub width: f32,
    /// Whether the layer is visible in the editor. The visibility setting in the layer panel (the eye symbol).
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub visible: bool,
}

/// Anchor (`GSAnchor`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Anchor {
    /// The name of the anchor.
    pub name: String,
    /// The position of the anchor as an x, y coordinate.
    #[serde(
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify"
    )]
    pub position: (f32, f32),
}

/// Background image (`GSBackgroundImage`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct BackgroundImage {
    /// The rectangle that defines the area to crop to in pixels, format: {{t,l},{b,r}}.
    #[serde(default)]
    pub crop: CropRect,
    /// The file path of the background image file. It is stored relative if close enough. Otherwise the full path.
    #[serde(rename = "imagePath", default)]
    pub image_path: String,
    /// Whether the background image is locked.
    #[serde(
        default,
        skip_serializing_if = "is_false",
        deserialize_with = "anything_to_bool"
    )]
    pub locked: bool,
    /// The affine transformation matrix applied to the background image.
    pub transform: Transform,
}

/// Affine transformation matrix in the form `{m11, m12, m21, m22, tX, tY}`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transform {
    /// m11 component of the transformation matrix.
    pub m11: f32,
    /// m12 component of the transformation matrix.
    pub m12: f32,
    /// m21 component of the transformation matrix.
    pub m21: f32,
    /// m22 component of the transformation matrix.
    pub m22: f32,
    /// Horizontal translation (tX) of the transformation matrix.
    pub t_x: f32,
    /// Vertical translation (tY) of the transformation matrix.
    pub t_y: f32,
}

/// Crop rectangle with origin and size
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CropRect {
    /// Top coordinate of the crop rectangle.
    pub top: i32,
    /// Left coordinate of the crop rectangle.
    pub left: i32,
    /// Bottom coordinate of the crop rectangle.
    pub bottom: i32,
    /// Right coordinate of the crop rectangle.
    pub right: i32,
}

/// Component (`GSComponent`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Component {
    /// The anchor that the component is aligned to. Should be indicated if connected to an anchor, especially if more than one possibility is available, e.g. in ligatures.
    #[serde(default, skip_serializing_if = "is_default")]
    pub anchor: Option<String>,
    /// The name of the glyph that the component references.
    #[serde(rename = "name")]
    pub component_glyph: String,
    /// The affine transformation matrix applied to the component.
    #[serde(default, skip_serializing_if = "is_default")]
    pub transform: Transform,
    /// The alignment of the component.
    #[serde(default, skip_serializing_if = "is_default")]
    pub alignment: i8,
    /// Whether automatic alignment is disabled for the component.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "disableAlignment"
    )]
    pub disable_alignment: bool,
}

/// Guide (`GSGuide`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Guide {
    /// The alignment of the guide.
    #[serde(default)]
    pub alignment: Orientation,
    /// The angle of the guide in degrees counter-clockwise.
    #[serde(default)]
    pub angle: f32,
    /// Whether the guide is locked.
    #[serde(default)]
    pub locked: bool,
    /// The position of the guide as an x, y coordinate.
    pub pos: (f32, f32),
    /// The scale of the guide.
    #[serde(
        default = "scale_unit",
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify",
        skip_serializing_if = "is_scale_unit"
    )]
    pub scale: (f32, f32),
}

/// PostScript hint (`GSHint`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Hint {
    /// Whether the hint is horizontal.
    #[serde(default, skip_serializing_if = "is_default")]
    pub horizontal: bool,
    /// The type of the hint.
    #[serde(default, rename = "type")]
    pub type_: String,
    /// The origin position of the hint.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify"
    )]
    pub origin: (f32, f32),
    /// The target node of the hint.
    #[serde(default, skip_serializing_if = "is_default")]
    pub target: HintTarget,
    /// The first other position of the hint.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify"
    )]
    pub other1: (f32, f32),
    /// The second other position of the hint.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify"
    )]
    pub other2: (f32, f32),
    /// The scale of the hint.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify"
    )]
    pub scale: (f32, f32),
    /// Whether the hint is a stem.
    #[serde(default, skip_serializing_if = "is_default")]
    pub stem: bool,
    /// The options for the hint.
    #[serde(default, skip_serializing_if = "is_default")]
    pub options: i8,
}

/// Hint target
///
/// Can be either a position coordinate or a label string.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum HintTarget {
    /// Position as an x, y coordinate.
    #[serde(
        serialize_with = "serialize_commify",
        deserialize_with = "deserialize_commify"
    )]
    Position((f32, f32)),
    /// Label as a string.
    Label(String),
}
impl Default for HintTarget {
    fn default() -> Self {
        HintTarget::Position((0.0, 0.0))
    }
}

/// Path (`GSPath`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Path {
    /// Whether the path is closed.
    #[serde(default, skip_serializing_if = "is_false")]
    pub closed: bool,
    /// The nodes of the path.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Node>,
}

/// Node (`GSNode`)
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The x coordinate of the node.
    pub x: f32,
    /// The y coordinate of the node.
    pub y: f32,
    /// The type of the node.
    pub node_type: NodeType,
}
