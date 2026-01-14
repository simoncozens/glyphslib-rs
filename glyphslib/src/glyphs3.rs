use std::collections::BTreeMap;

use openstep_plist::{Dictionary, Plist};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, OneOrMany};

use crate::{
    common::{
        Color, CustomParameter, Feature, FeatureClass, FeaturePrefix, InstanceFactors, Kerning,
        NodeType, Orientation, Version,
    },
    serde::{
        bool_true, deserialize_export_type, int_to_bool, is_default, is_false, is_scale_unit,
        is_true, scale_unit, SerializeAsTuple,
    },
};

pub(crate) fn version_two() -> i32 {
    2
}

/// Glyphs file format version 3 document
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Glyphs3 {
    /// The build number of Glyphs used to save the file. Example: `"3210"`.
    #[serde(
        rename = ".appVersion",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub app_version: String,
    /// The version of the file format. If unset, the file is considered to be version 1 as used by Glyphs 1 and Glyphs 2.
    #[serde(rename = ".formatVersion", default = "version_two")]
    pub format_version: i32,
    /// The strings of the Edit View tabs. Omitted when the `Write DisplayStrings` custom parameter is set to false.
    #[serde(rename = "DisplayStrings", skip_serializing_if = "is_default", default)]
    pub display_strings: Vec<String>,
    /// The designspace variation axes of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub axes: Vec<Axis>,
    /// The OpenType layout classes of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<FeatureClass>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub glyphs: Vec<Glyph>,
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
    /// The left-to-right kerning of the font.
    #[serde(rename = "kerningLTR", default, skip_serializing_if = "is_default")]
    pub kerning: Kerning,
    /// The right-to-left kerning of the font.
    #[serde(rename = "kerningRTL", default, skip_serializing_if = "is_default")]
    pub kerning_rtl: Kerning,
    /// The vertical kerning of the font.
    #[serde(
        rename = "kerningVertical",
        default,
        skip_serializing_if = "is_default"
    )]
    pub kerning_vertical: Kerning,
    /// The metrics of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<Metric>,
    /// The note about the font.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    /// The numbers of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub numbers: Vec<Number>,
    /// The properties of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    /// The settings of the font.
    #[serde(default, skip_serializing_if = "is_default")]
    pub settings: Settings,
    /// The stems of the font.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stems: Vec<Stem>,
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

/// Number metric
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Number {
    /// The name of the number.
    pub name: String,
}

/// Metric definition (`GSMetric`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Metric {
    /// The filter of the metric limiting the scope of the metric to a subset of glyphs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// The name of the metric.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The type of the metric.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub metric_type: Option<MetricType>,
}

/// Metric type
#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
pub enum MetricType {
    /// Ascender metric
    #[serde(rename = "ascender")]
    Ascender,
    /// Cap height metric
    #[serde(rename = "cap height")]
    CapHeight,
    /// Slant height metric
    #[serde(rename = "slant height")]
    SlantHeight,
    /// X-height metric
    #[serde(rename = "x-height")]
    XHeight,
    /// Mid height metric
    #[serde(rename = "midHeight")]
    MidHeight,
    /// Top height metric
    #[serde(rename = "topHeight")]
    TopHeight,
    /// Body height metric
    #[serde(rename = "bodyHeight")]
    BodyHeight,
    /// Descender metric
    #[serde(rename = "descender")]
    Descender,
    /// Baseline metric
    #[serde(rename = "baseline")]
    Baseline,
    /// Italic angle metric
    #[serde(rename = "italic angle")]
    ItalicAngle,
}

/// Font settings
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Settings {
    /// Whether automatic alignment of components is disabled.
    #[serde(
        rename = "disablesAutomaticAlignment",
        default,
        skip_serializing_if = "is_default"
    )]
    pub disables_automatic_alignment: bool,
    /// Whether to use production names instead of nice names.
    #[serde(
        rename = "disablesNiceNames",
        default,
        skip_serializing_if = "is_default"
    )]
    pub disables_nice_names: bool,
    /// The main grid length.
    #[serde(rename = "gridLength", skip_serializing_if = "Option::is_none")]
    pub grid_length: Option<i32>,
    /// The grid sub-division size.
    #[serde(rename = "gridSubDivision", skip_serializing_if = "Option::is_none")]
    pub grid_sub_division: Option<i32>,
    /// The standard keyboard increment.
    #[serde(rename = "keyboardIncrement", skip_serializing_if = "Option::is_none")]
    pub keyboard_increment: Option<f32>,
    /// The keyboard increment when holding the Shift key.
    #[serde(
        rename = "keyboardIncrementBig",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyboard_increment_big: Option<f32>,
    /// The keyboard increment when holding both the Shift and Command key.
    #[serde(
        rename = "keyboardIncrementHuge",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyboard_increment_huge: Option<f32>,
}

/// Axis definition (`GSAxis`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Axis {
    /// Whether the axis is considered to be hidden from the font user.
    #[serde(default, skip_serializing_if = "is_default")]
    pub hidden: bool,
    /// The user-facing name of the axis.
    pub name: String,
    /// The OpenType tag of the axis. Must be unique within the font.
    pub tag: String,
}

/// Font master (`GSFontMaster`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Master {
    /// The designspace location of the master.
    #[serde(rename = "axesValues", default, skip_serializing_if = "Vec::is_empty")]
    pub axes_values: Vec<f32>,
    /// The custom parameters of the master.
    #[serde(
        rename = "customParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    /// The global guides of the master.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guides: Vec<Guide>,
    /// The name of the icon that represents the master. Generally omitted when equal to `Regular`, or equal to the default icon name of the master.
    #[serde(rename = "iconName", default, skip_serializing_if = "String::is_empty")]
    pub icon_name: String,
    /// The unique identifier of the master.
    pub id: String,
    /// The metric values of the master.
    #[serde(
        rename = "metricValues",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_values: Vec<MetricValue>,
    /// The name of the master.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The number values of the master.
    #[serde(
        rename = "numberValues",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub number_values: Vec<f32>,
    /// The properties of the master.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    /// The stem values of the master.
    #[serde(rename = "stemValues", default, skip_serializing_if = "Vec::is_empty")]
    pub stem_values: Vec<f32>,
    /// Custom data associated with the master.
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    /// Whether the master is visible in the preview.
    #[serde(default, skip_serializing_if = "is_default")]
    pub visible: bool,
}

/// Metric value store (`GSMetricStore`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct MetricValue {
    /// The overshoot of the metric value.
    #[serde(default, skip_serializing_if = "is_default")]
    pub over: f32,
    /// The offset from the baseline of the metric value.
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: f32,
}

/// Glyph definition (`GSGlyph`)
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Glyph {
    /// The kerning group of the bottom side of the glyph.
    #[serde(rename = "kernBottom", skip_serializing_if = "Option::is_none")]
    pub kern_bottom: Option<String>,

    /// The case of the glyph. If unset, then the case is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub case: String,
    /// The category of the glyph. If unset, then the category is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The color label of the glyph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// The writing direction of the glyph. If unset, then the writing direction is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Whether the glyph is exported.
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub export: bool,
    /// The name of the glyph.
    #[serde(rename = "glyphname")]
    pub name: String,
    /// The kerning group of the left side of the glyph.
    #[serde(rename = "kernLeft", skip_serializing_if = "Option::is_none")]
    pub kern_left: Option<String>,
    /// The kerning group of the right side of the glyph.
    #[serde(rename = "kernRight", skip_serializing_if = "Option::is_none")]
    pub kern_right: Option<String>,
    /// The kerning group of the top side of the glyph.
    #[serde(rename = "kernTop", skip_serializing_if = "Option::is_none")]
    pub kern_top: Option<String>,
    /// The date and time of the last change of the glyph. Example: `"2023-02-25 14:46:49 +0000"`.
    #[serde(rename = "lastChange", skip_serializing_if = "Option::is_none")]
    pub last_change: Option<String>,
    /// The layers of the glyph.
    pub layers: Vec<Layer>,
    /// Whether the glyph is locked.
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    /// The bottom metrics key of the glyph.
    #[serde(default, skip_serializing_if = "is_default", rename = "metricBottom")]
    pub metric_bottom: Option<String>,
    /// The left metrics key of the glyph.
    #[serde(default, skip_serializing_if = "is_default", rename = "metricLeft")]
    pub metric_left: Option<String>,
    /// The right metrics key of the glyph.
    #[serde(default, skip_serializing_if = "is_default", rename = "metricRight")]
    pub metric_right: Option<String>,
    /// The top metrics key of the glyph.
    #[serde(default, skip_serializing_if = "is_default", rename = "metricTop")]
    pub metric_top: Option<String>,
    /// The vertical width metrics key of the glyph.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "metricVertWidth"
    )]
    pub metric_vert_width: Option<String>,
    /// The width metrics key of the glyph.
    #[serde(default, skip_serializing_if = "is_default", rename = "metricWidth")]
    pub metric_width: Option<String>,
    /// A string serving as a description or comment about the glyph.
    #[serde(default, skip_serializing_if = "is_default")]
    pub note: String,
    /// A list of the Smart Glyph properties and their top/bottom values.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "partsSettings"
    )]
    pub smart_component_settings: Vec<SmartComponentSetting>,
    /// The production name of the glyph. If unset, then the production name is based on a glyph data lookup based on the glyph name or the Unicode code point.
    #[serde(default, skip_serializing_if = "is_default")]
    pub production: Option<String>,
    /// The script of the glyph. If unset, then the script is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub script: Option<String>,
    /// The sort name of the glyph. If unset, then the sort name is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "is_default", rename = "sortName")]
    pub sort_name: Option<String>,
    /// The sort name of the glyph used in the *Keep Alternates Next to Base Glyph* display mode. If unset, then the sort name is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "is_default", rename = "sortNameKeep")]
    pub sort_name_keep: Option<String>,
    /// The subcategory of the glyph. If unset, then the subcategory is based on a glyph data lookup based on the glyph name.
    #[serde(default, skip_serializing_if = "is_default", rename = "subCategory")]
    pub subcategory: Option<String>,
    /// The tags of the glyph, sorted lexicographically.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// The Unicode code points of the glyph.
    #[serde_as(
        deserialize_as = "OneOrMany<_>",
        serialize_as = "SerializeAsTuple<u32>"
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unicode: Vec<u32>,
    /// Custom data associated with the glyph.
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
}

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

/// Layer definition (`GSLayer`)
///
/// We manually serialize this because background layers serialize differently,
/// and I don't want to have a separate BackgroundLayer struct.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Layer {
    /// The anchors of the layer.
    #[serde(default)]
    pub anchors: Vec<Anchor>,
    /// The annotations of the layer.
    #[serde(default)]
    pub annotations: Vec<Dictionary>,
    /// The unique identifier of the associated master. Omitted when equal to the layer ID.
    #[serde(rename = "associatedMasterId", default)]
    pub associated_master_id: Option<String>,
    /// The attributes of the layer.
    #[serde(default)]
    pub attr: Dictionary,
    /// The background layer.
    #[serde(default)]
    pub background: Option<Box<Layer>>,
    /// The background image of the layer.
    #[serde(rename = "backgroundImage", default)]
    pub background_image: Option<BackgroundImage>,
    /// The color label of the layer.
    #[serde(default)]
    pub color: Option<Color>,
    /// The guides of the layer.
    #[serde(default)]
    pub guides: Vec<Guide>,
    /// The hints of the layer.
    #[serde(default)]
    pub hints: Vec<Dictionary>, // This thing's an absolute minefield
    /// The unique ID of the layer. Matches the master ID when the layer is a master layer.
    #[serde(rename = "layerId", default)]
    // Not required for background layers
    pub layer_id: String,
    /// The bottom metrics key of the layer.
    #[serde(rename = "metricBottom", default)]
    pub metric_bottom: Option<String>,
    /// The left metrics key of the layer.
    #[serde(rename = "metricLeft", default)]
    pub metric_left: Option<String>,
    /// The right metrics key of the layer.
    #[serde(rename = "metricRight", default)]
    pub metric_right: Option<String>,
    /// The top metrics key of the layer.
    #[serde(rename = "metricTop", default)]
    pub metric_top: Option<String>,
    /// The vertical width metrics key of the layer.
    #[serde(
        rename = "metricVertWidth",
        default,
        skip_serializing_if = "is_default"
    )]
    pub metric_vert_width: Option<String>,
    /// The width metrics key of the layer.
    #[serde(rename = "metricWidth", default)]
    pub metric_width: Option<String>,
    /// The name of the layer. Master layers and other special layers display a name in the Glyphs UI that is derived from the layers role. These derived names are not written to the file.
    #[serde(default)]
    pub name: Option<String>,
    /// The Smart Glyph setting of the layer. The keys are the property names. The values are either `1` if the layer corresponds to the bottom value of the property or `2` if the layer corresponds to the top value of the property.
    #[serde(rename = "partSelection", default)]
    pub part_selection: BTreeMap<String, u8>,
    /// The shapes of the layer.
    #[serde(default)]
    pub shapes: Vec<Shape>,
    /// Custom data associated with the layer.
    #[serde(rename = "userData", default)]
    pub user_data: Dictionary,
    /// The vertical origin of the layer.
    #[serde(rename = "vertOrigin", default)]
    pub vert_origin: Option<f32>,
    /// The vertical width of the layer.
    #[serde(rename = "vertWidth", default)]
    pub vert_width: Option<f32>,
    /// Whether the layer is visible.
    #[serde(default = "bool_true")]
    pub visible: bool,
    /// The width of the layer.
    #[serde(default)]
    pub width: f32,
}

/// Anchor definition (`GSAnchor`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Anchor {
    /// Whether the anchor is locked.
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    /// The name of the anchor.
    pub name: String,
    /// The orientation of the anchor.
    #[serde(default, skip_serializing_if = "is_default")]
    pub orientation: Orientation,
    #[serde(default, skip_serializing_if = "is_default")]
    /// The position of the anchor.
    pub pos: (f32, f32),
    /// Custom data associated with the anchor.
    #[serde(default, rename = "userData", skip_serializing_if = "is_default")]
    pub user_data: Option<Dictionary>,
}

/// Background image (`GSImage`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BackgroundImage {
    /// The rotation angle of the image in degrees clockwise.
    #[serde(default, skip_serializing_if = "is_default")]
    pub angle: f32,
    /// The cropped frame of the image, specified as the crop origin X/Y and size width/height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crop: Option<(f32, f32, f32, f32)>,
    /// The file path of the image file relative to the document file.
    #[serde(rename = "imagePath")]
    pub image_path: String,
    /// Whether the image is locked.
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    /// The position of the image.
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: (f32, f32),
    /// The scale factor of the image.
    #[serde(default = "scale_unit", skip_serializing_if = "is_scale_unit")]
    pub scale: (f32, f32),
}

/// Guide type (`GSGuideType`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Copy)]
pub enum GuideType {
    /// Line guide
    #[default]
    Line,
    /// Circle guide
    Circle,
    /// Rectangle guide
    Rect,
}

/// Guide definition (`GSGuide`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Guide {
    /// The angle at which the guide is drawn in degrees clockwise.
    #[serde(default, skip_serializing_if = "is_default")]
    pub angle: f32,
    /// Whether the angle of the guide is locked.
    #[serde(default, skip_serializing_if = "is_default", rename = "lockAngle")]
    pub lock_angle: bool,
    /// Whether the guide is locked.
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    /// The orientation of the guide.
    #[serde(default, skip_serializing_if = "is_default")]
    pub orientation: Orientation,
    /// The position of the guide.
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: (f32, f32),
    /// Whether the measurement of the guide is shown
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "showMeasurement"
    )]
    pub show_measurement: bool,
    /// The size of the guide.
    #[serde(default = "scale_unit", skip_serializing_if = "is_scale_unit")]
    pub size: (f32, f32),
    /// The type of the guide
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub guide_type: GuideType,
    /// Custom data associated with the guide.
    #[serde(default, rename = "userData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Dictionary>,
}

/// Shape - either a path or a component
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Shape {
    /// Component reference
    Component(Component),
    /// Outline path
    Path(Path),
}

/// Path definition (`GSPath`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Path {
    /// The attributes of the path.
    #[serde(default, skip_serializing_if = "is_default")]
    pub attr: Dictionary,
    /// Whether the path is closed.
    #[serde(default, deserialize_with = "int_to_bool")]
    pub closed: bool,
    /// The nodes of the path.
    pub nodes: Vec<Node>,
}

/// Path node (`GSNode`)
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The x-coordinate of the node.
    pub x: f32,
    /// The y-coordinate of the node.
    pub y: f32,
    /// The type of the node.
    pub node_type: NodeType,
    /// Custom data associated with the node.
    pub user_data: Option<Dictionary>,
}

/// Component reference (`GSComponent`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Component {
    /// Controls the automatic alignment of the component. `-1`: disabled (no alignment), `0`: default (alignment is based on context), `1`: force alignment (align regardless of context), `3`: horizontal alignment (align horizontally, but allow for manual vertical placement).
    #[serde(default, skip_serializing_if = "is_default")]
    pub alignment: i8,
    /// The name of the attachment anchor. Set to specify a specific anchor when there are multiple candidates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Undocumented anchor targeting.
    #[serde(default, rename = "anchorTo", skip_serializing_if = "Option::is_none")]
    pub anchor_to: Option<String>,
    /// The rotation angle of the component in degrees clockwise.
    #[serde(default, skip_serializing_if = "is_default")]
    pub angle: f32,
    /// The attributes of the component.
    #[serde(default, skip_serializing_if = "is_default")]
    pub attr: Dictionary,
    /// Whether the component is locked.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "int_to_bool"
    )]
    pub locked: bool,
    /// The ID of the master from which the component is derived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    /// The orientation of the component.
    #[serde(default, skip_serializing_if = "is_default")]
    pub orientation: Orientation,
    /// The Smart Component settings of the component, mapping property names to values.
    #[serde(rename = "piece", default, skip_serializing_if = "is_default")]
    pub smart_component_location: BTreeMap<String, f32>,
    /// The position (translation transform) of the component.
    #[serde(default, rename = "pos", skip_serializing_if = "is_default")]
    pub position: (f32, f32),
    /// The name of the referenced glyph.
    #[serde(rename = "ref")]
    pub component_glyph: String,
    /// The scale transform of the component.
    #[serde(default = "scale_unit", skip_serializing_if = "is_scale_unit")]
    pub scale: (f32, f32),
    /// The slant transform of the component.
    #[serde(default, skip_serializing_if = "is_default")]
    pub slant: (f32, f32),
    /// Custom data associated with the component.
    #[serde(default, rename = "userData", skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
}

/// Instance definition (`GSInstance`)
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Instance {
    /// The internal axis locations of the instance. These values are also used for the external axis locations, if no external axis locations are specified separately.
    #[serde(default, rename = "axesValues", skip_serializing_if = "is_default")]
    pub axes_values: Vec<f32>,
    /// The custom parameters of the instance.
    #[serde(
        default,
        rename = "customParameters",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    /// Whether the instance is exported.
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub exports: bool,
    /// The interpolation factors where the keys are the master IDs.
    #[serde(
        default,
        rename = "instanceInterpolations",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub instance_interpolations: BTreeMap<String, InstanceFactors>,
    /// Whether the instance is bold.
    #[serde(default, rename = "isBold", skip_serializing_if = "is_false")]
    pub is_bold: bool,
    /// Whether the instance is italic.
    #[serde(default, rename = "isItalic", skip_serializing_if = "is_false")]
    pub is_italic: bool,
    /// The name of the style-linked instance.
    #[serde(default, rename = "linkStyle", skip_serializing_if = "Option::is_none")]
    pub link_style: Option<String>,
    /// Whether the `instanceInterpolations` values are used. Otherwise, the values are calculated from the axis values.
    #[serde(
        default,
        rename = "manualInterpolation",
        skip_serializing_if = "is_false"
    )]
    pub manual_interpolation: bool,
    /// The style name of the instance. Examples: `"Regular"`, `"Bold"`, `"Italic"`, `"Bold Italic"`.
    pub name: String,
    /// The properties of the instance.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    /// The type of the instance.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "deserialize_export_type"
    )]
    pub export_type: ExportType,
    /// Custom data associated with the instance.
    #[serde(default, rename = "userData", skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    /// The weight class of the instance.
    #[serde(
        default,
        rename = "weightClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub weight_class: Option<Plist>, // String or integer
    /// The width class of the instance.
    #[serde(
        default,
        rename = "widthClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub width_class: Option<Plist>,
}

/// Instance export type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Copy)]
pub enum ExportType {
    /// Static instance
    #[default]
    Static,
    /// Variable instance
    #[serde(rename = "variable")]
    Variable,
}

/// Font property (`GSInfoProperty`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Property {
    /// Singular (non-localized) property
    SingularProperty {
        /// Property key
        key: SingularPropertyKey,
        /// Property value
        value: String,
    },
    /// Localized property with multiple language variants
    LocalizedProperty {
        /// Property key
        key: LocalizedPropertyKey,
        /// Localized values
        values: Vec<LocalizedValue>,
    },
    /// Unrecognized property format
    Junk(Plist),
}

impl Property {
    /// Create a singular property
    pub fn singular(key: SingularPropertyKey, value: String) -> Self {
        Property::SingularProperty { key, value }
    }
    /// Create a localized property with a default language value
    pub fn localized_with_default(key: LocalizedPropertyKey, value: String) -> Self {
        Property::LocalizedProperty {
            key,
            values: vec![LocalizedValue {
                language: "dflt".to_string(),
                value,
            }],
        }
    }
}

/// Localized property key
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum LocalizedPropertyKey {
    /// Family names
    #[serde(rename = "familyNames")]
    FamilyNames,
    /// Copyright notices
    #[serde(rename = "copyrights")]
    Copyrights,
    /// Designer names
    #[serde(rename = "designers")]
    Designers,
    /// Manufacturer names
    #[serde(rename = "manufacturers")]
    Manufacturers,
    /// License texts
    #[serde(rename = "licenses")]
    Licenses,
    /// Trademark notices
    #[serde(rename = "trademarks")]
    Trademarks,
    /// Font descriptions
    #[serde(rename = "descriptions")]
    Descriptions,
    /// Sample texts
    #[serde(rename = "sampleTexts")]
    SampleTexts,
    /// Compatible full names
    #[serde(rename = "compatibleFullNames")]
    CompatibleFullNames,
    /// Style names
    #[serde(rename = "styleNames")]
    StyleNames,
}

/// Singular (non-localized) property key
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SingularPropertyKey {
    /// Designer name
    #[serde(rename = "designer")]
    Designer,
    /// Manufacturer name
    #[serde(rename = "manufacturer")]
    Manufacturer,
    /// Designer URL
    #[serde(rename = "designerURL")]
    DesignerUrl,
    /// Manufacturer URL
    #[serde(rename = "manufacturerURL")]
    ManufacturerUrl,
    /// License URL
    #[serde(rename = "licenseURL")]
    LicenseUrl,
    /// PostScript full name
    #[serde(rename = "postscriptFullName")]
    PostscriptFullName,
    /// PostScript font name
    #[serde(rename = "postscriptFontName")]
    PostscriptFontName,
    /// WWS family name
    #[serde(rename = "WWSFamilyName")]
    WwsFamilyName,
    /// Version string
    #[serde(rename = "versionString")]
    VersionString,
    /// Vendor ID
    #[serde(rename = "vendorID")]
    VendorID,
    /// Unique ID
    #[serde(rename = "uniqueID")]
    UniqueID,
}

/// Localized string value (`GSInfoValue`)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LocalizedValue {
    /// The language tag of the string value. The tag is based on the OpenType Language System Tags but omits trailing whitespace. Examples: `"dflt"`, `"DEU"`.
    pub language: String,
    /// The localized string value.
    pub value: String,
}

/// Stem definition
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stem {
    /// Whether the stem is a horizontal stem.
    #[serde(default, skip_serializing_if = "is_default")]
    pub horizontal: bool,
    /// The name of the stem.
    name: String,
}

#[cfg(test)]
mod tests {
    use openstep_plist::de::Deserializer;

    use super::*;

    #[test]
    fn test_guide() {
        let data = r#"
        {
            angle = 45.0;
            lockAngle = 1;
            locked = 0;
            orientation = left;
            pos = (100.0, 200.0);
            showMeasurement = 1;
            size = (300.0, 400.0);
            type = "Line";
            userData = {
                customKey = "customValue";
            };
        }
        "#;
        let plist = Plist::parse(data).expect("Failed to parse plist");
        let deserializer = &mut Deserializer::from_plist(&plist);
        let guide: Guide =
            serde_path_to_error::deserialize(deserializer).expect("Failed to deserialize Guide");

        assert_eq!(guide.angle, 45.0);
        assert!(guide.lock_angle);
        assert!(!guide.locked);
        assert_eq!(guide.orientation, Orientation::Left);
        assert_eq!(guide.pos, (100.0, 200.0));
        assert!(guide.show_measurement);
        assert_eq!(guide.size, (300.0, 400.0));
        assert_eq!(guide.guide_type, GuideType::Line);
        assert!(guide.user_data.is_some());
    }
}
