use std::collections::BTreeMap;

use openstep_plist::{Dictionary, Plist};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, OneOrMany};

use crate::{
    common::{
        bool_true, is_default, is_false, is_scale_unit, is_true, scale_unit, Color,
        CustomParameter, Feature, FeatureClass, FeaturePrefix, GuideAlignment, InstanceFactors,
        Kerning, NodeType, Version,
    },
    serde::{deserialize_export_type, int_to_bool, SerializeAsTuple},
};

pub(crate) fn version_two() -> i32 {
    2
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Glyphs3 {
    /// The build number of the app
    #[serde(
        rename = ".appVersion",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub app_version: String,
    /// Set to 3 for version 3. If that key is missing assume version 2.
    #[serde(rename = ".formatVersion", default = "version_two")]
    pub format_version: i32,
    /// List of strings used in the edit window
    #[serde(rename = "DisplayStrings", skip_serializing_if = "is_default", default)]
    pub display_strings: Vec<String>,
    /// The interpolation axes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub axes: Vec<Axis>,
    /// OpenType classes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<FeatureClass>,
    /// Font-wide custom parameters
    #[serde(
        default,
        rename = "customParameters",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    /// Font creation date. Format `2014-01-29 14:14:38 +0000`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub date: String,
    /// The family name of the font
    #[serde(rename = "familyName")]
    pub family_name: String,
    /// OpenType feature code before the class definitions.
    #[serde(
        default,
        rename = "featurePrefixes",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub feature_prefixes: Vec<FeaturePrefix>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<Feature>,
    /// Masters
    #[serde(rename = "fontMaster", skip_serializing_if = "Vec::is_empty", default)]
    pub masters: Vec<Master>,
    /// Glyphs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub glyphs: Vec<Glyph>,
    /// Instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<Instance>,
    #[serde(
        rename = "keepAlternatesTogether",
        default,
        skip_serializing_if = "is_default"
    )]
    pub keep_alternates_together: bool,
    /// Three-level dict containing a float as value.
    #[serde(rename = "kerningLTR", default, skip_serializing_if = "is_default")]
    pub kerning: Kerning,
    #[serde(rename = "kerningRTL", default, skip_serializing_if = "is_default")]
    pub kerning_rtl: Kerning,
    #[serde(
        rename = "kerningVertical",
        default,
        skip_serializing_if = "is_default"
    )]
    pub kerning_vertical: Kerning,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<Metric>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub numbers: Vec<Number>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub settings: Settings,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stems: Vec<Stem>,
    #[serde(rename = "unitsPerEm")]
    pub units_per_em: i32,
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    #[serde(flatten, default, skip_serializing_if = "is_default")]
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Number {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Metric {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub metric_type: Option<MetricType>,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
pub enum MetricType {
    #[serde(rename = "ascender")]
    Ascender,
    #[serde(rename = "cap height")]
    CapHeight,
    #[serde(rename = "slant height")]
    SlantHeight,
    #[serde(rename = "x-height")]
    XHeight,
    #[serde(rename = "midHeight")]
    MidHeight,
    #[serde(rename = "topHeight")]
    TopHeight,
    #[serde(rename = "bodyHeight")]
    BodyHeight,
    #[serde(rename = "descender")]
    Descender,
    #[serde(rename = "baseline")]
    Baseline,
    #[serde(rename = "italic angle")]
    ItalicAngle,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Settings {
    #[serde(
        rename = "disablesAutomaticAlignment",
        default,
        skip_serializing_if = "is_default"
    )]
    pub disables_automatic_alignment: bool,
    #[serde(
        rename = "disablesNiceNames",
        default,
        skip_serializing_if = "is_default"
    )]
    pub disables_nice_names: bool,
    #[serde(rename = "gridLength", skip_serializing_if = "Option::is_none")]
    pub grid_length: Option<i32>,
    #[serde(rename = "gridSubDivision", skip_serializing_if = "Option::is_none")]
    pub grid_sub_division: Option<i32>,
    #[serde(rename = "keyboardIncrement", skip_serializing_if = "Option::is_none")]
    pub keyboard_increment: Option<f32>,
    #[serde(
        rename = "keyboardIncrementBig",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyboard_increment_big: Option<f32>,
    #[serde(
        rename = "keyboardIncrementHuge",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyboard_increment_huge: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Axis {
    /// If the axis should be visible in the UI.
    #[serde(default, skip_serializing_if = "is_default")]
    pub hidden: bool,
    /// The name of the axis (e.g. `Weight``)
    pub name: String,
    /// The axis tag (e.g. `wght`)
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Master {
    /// A list of float values storing the axis coordinate for each axis
    ///
    /// Axis settings are stored in the Font object.
    #[serde(rename = "axesValues", default, skip_serializing_if = "Vec::is_empty")]
    pub axes_values: Vec<f32>,
    /// Master-wide custom parameters
    #[serde(
        rename = "customParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guides: Vec<Guide>,
    /// Stores the selected master icon
    #[serde(rename = "iconName", default, skip_serializing_if = "String::is_empty")]
    pub icon_name: String,
    /// A unique id that connects the layers (associated ID) with the master
    pub id: String,
    /// The metrics values
    ///
    /// Metrics settings are stored in the font object.
    #[serde(
        rename = "metricValues",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_values: Vec<MetricValue>,
    /// The name of the master
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// A list of floats, number settings are stored in the font object.
    #[serde(
        rename = "numberValues",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub number_values: Vec<f32>,
    /// Properties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    /// The stem values
    #[serde(rename = "stemValues", default, skip_serializing_if = "Vec::is_empty")]
    pub stem_values: Vec<f32>,
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    #[serde(default, skip_serializing_if = "is_default")]
    pub visible: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct MetricValue {
    #[serde(default, skip_serializing_if = "is_default")]
    pub over: f32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: f32,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Glyph {
    ///  Bottom kerning group
    #[serde(rename = "kernBottom", skip_serializing_if = "Option::is_none")]
    pub kern_bottom: Option<String>,

    /// The 'case' of the glyph when manually set.
    ///
    /// Possible values: "noCase", "upper", "lower", "smallCaps", "other".
    /// This could be used to specify 'height' of default numbers (lining vs old style)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub case: String,
    /// Manually set category
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The color of the glyph in the interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// The writing direction when manually set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Export
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub export: bool,
    /// The glyph name
    #[serde(rename = "glyphname")]
    pub name: String,
    /// Left kerning group
    #[serde(rename = "kernLeft", skip_serializing_if = "Option::is_none")]
    pub kern_left: Option<String>,
    /// Right kerning group
    #[serde(rename = "kernRight", skip_serializing_if = "Option::is_none")]
    pub kern_right: Option<String>,
    /// Top kerning group
    #[serde(rename = "kernTop", skip_serializing_if = "Option::is_none")]
    pub kern_top: Option<String>,
    /// Format 2014-01-29 14:14:38 +0000
    #[serde(rename = "lastChange", skip_serializing_if = "Option::is_none")]
    pub last_change: Option<String>,
    pub layers: Vec<Layer>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    #[serde(default, skip_serializing_if = "is_default", rename = "metricBottom")]
    pub metric_bottom: Option<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "metricLeft")]
    pub metric_left: Option<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "metricRight")]
    pub metric_right: Option<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "metricTop")]
    pub metric_top: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "metricVertWidth"
    )]
    pub metric_vert_width: Option<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "metricWidth")]
    pub metric_width: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "partsSettings"
    )]
    pub smart_component_settings: Vec<SmartComponentSetting>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub production: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub script: Option<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "subCategory")]
    pub subcategory: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde_as(
        deserialize_as = "OneOrMany<_>",
        serialize_as = "SerializeAsTuple<u32>"
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unicode: Vec<u32>,
    /// User data
    #[serde(rename = "userData", default, skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SmartComponentSetting {
    #[serde(default, rename = "bottomValue")]
    bottom_value: i32,
    #[serde(default, rename = "topValue")]
    top_value: i32,
    name: String,
}

// We manually serialize this because background layers serialize differently,
// and I don't want to have a separate BackgroundLayer struct.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Layer {
    #[serde(default)]
    pub anchors: Vec<Anchor>,
    #[serde(default)]
    pub annotations: Vec<Dictionary>,
    /// ID of the master the layer is linked to
    ///
    /// Not present if it equals layerID, i.e. if the layer is in use as master.
    #[serde(rename = "associatedMasterId", default)]
    pub associated_master_id: Option<String>,
    #[serde(default)]
    pub attr: Dictionary,
    #[serde(default)]
    pub background: Option<Box<Layer>>,
    #[serde(rename = "backgroundImage", default)]
    pub background_image: Option<BackgroundImage>,
    #[serde(default)]
    pub color: Option<Color>,
    #[serde(default)]
    pub guides: Vec<Guide>,
    #[serde(default)]
    pub hints: Vec<Dictionary>, // This thing's an absolute minefield
    /// The unique id of the layer
    #[serde(rename = "layerId", default)]
    // Not required for background layers
    pub layer_id: String,
    /// Bottom metric key
    #[serde(rename = "metricBottom", default)]
    pub metric_bottom: Option<String>,
    /// Left metric key
    #[serde(rename = "metricLeft", default)]
    pub metric_left: Option<String>,
    /// Right metric key
    #[serde(rename = "metricRight", default)]
    pub metric_right: Option<String>,
    /// Top metric key
    #[serde(rename = "metricTop", default)]
    pub metric_top: Option<String>,
    /// Vertical width metric key
    #[serde(
        rename = "metricVertWidth",
        default,
        skip_serializing_if = "is_default"
    )]
    pub metric_vert_width: Option<String>,
    /// Horizontal width metric key
    #[serde(rename = "metricWidth", default)]
    pub metric_width: Option<String>,
    /// The name of the layer.
    ///
    /// Only stored for non-master layers (this is changed in 2.3, before the master names where stored)
    #[serde(default)]
    pub name: Option<String>,
    /// Smart component part selection
    #[serde(default)]
    pub part_selection: BTreeMap<String, u8>,
    /// Shapes
    ///
    /// Can be paths or components
    #[serde(default)]
    pub shapes: Vec<Shape>,
    /// User data
    #[serde(rename = "userData", default)]
    pub user_data: Dictionary,
    /// Offset from default (ascender)
    #[serde(rename = "vertOrigin", default)]
    pub vert_origin: Option<f32>,
    /// Vertical width
    ///
    /// Only stored if other than the default (ascender+descender)
    #[serde(rename = "vertWidth", default)]
    pub vert_width: Option<f32>,
    /// The visibility setting in the layer panel (the eye symbol).
    #[serde(default = "bool_true")]
    pub visible: bool,
    /// Layer width
    ///
    /// Should be skipped if it's zero and we are a background layer.
    #[serde(default)]
    pub width: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Anchor {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: (f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BackgroundImage {
    /// The angle
    #[serde(default)]
    pub angle: f32,
    /// The image path
    #[serde(rename = "imagePath")]
    pub image_path: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    /// The image scale
    #[serde(default = "scale_unit", skip_serializing_if = "is_scale_unit")]
    pub scale: (f32, f32),
    /// The origin
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: (f32, f32),
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Guide {
    #[serde(default, skip_serializing_if = "is_default")]
    pub alignment: GuideAlignment,
    #[serde(default, skip_serializing_if = "is_default")]
    pub angle: f32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub locked: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pos: (f32, f32),
    #[serde(default = "scale_unit", skip_serializing_if = "is_scale_unit")]
    pub scale: (f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Shape {
    Component(Component),
    Path(Path),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Path {
    #[serde(default, skip_serializing_if = "is_default")]
    pub attr: Dictionary,
    // Because we are using an untagged enum, types need to match precisely
    #[serde(default, deserialize_with = "int_to_bool")]
    pub closed: bool,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub node_type: NodeType,
    pub user_data: Option<Dictionary>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Component {
    #[serde(default, skip_serializing_if = "is_default")]
    /// Controls the automatic alignment of this component.
    ///
    /// -1 disables alignment, 1 forces it for glyph that are usually not aligned.
    pub alignment: i8,
    /// Should be indicated if connected to an anchor, especially if more than one possibility is available, e.g. in ligatures
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// A completely undocumented thing.
    #[serde(default, rename = "anchorTo", skip_serializing_if = "Option::is_none")]
    pub anchor_to: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub angle: f32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub attr: Dictionary,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "int_to_bool"
    )]
    pub locked: bool,
    /// If left, center or right aligned
    #[serde(default, skip_serializing_if = "is_default")]
    pub orientation: i8,
    /// Smart component location
    #[serde(rename = "piece", default, skip_serializing_if = "is_default")]
    pub smart_component_location: BTreeMap<String, f32>,
    /// The position
    #[serde(default, rename = "pos", skip_serializing_if = "is_default")]
    pub position: (f32, f32),
    /// The name of the linked glyph
    #[serde(rename = "ref")]
    pub component_glyph: String,
    #[serde(default = "scale_unit", skip_serializing_if = "is_scale_unit")]
    pub scale: (f32, f32),
    #[serde(default, rename = "userData", skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Instance {
    /// A list of float values storing the axis coordinate for each axis
    ///
    /// Axis settings are stored in the font object.
    #[serde(default, rename = "axesValues", skip_serializing_if = "is_default")]
    pub axes_values: Vec<f32>,
    #[serde(
        default,
        rename = "customParameters",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_parameters: Vec<CustomParameter>,
    #[serde(default = "bool_true", skip_serializing_if = "is_true")]
    pub exports: bool,
    /// Keys are master IDs, values are the factors for that master.
    #[serde(
        default,
        rename = "instanceInterpolations",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub instance_interpolations: BTreeMap<String, InstanceFactors>,
    /// For style linking. Always set to 1, otherwise omit the key.
    #[serde(default, rename = "isBold", skip_serializing_if = "is_false")]
    pub is_bold: bool,
    /// For style linking. Always set to 1, otherwise omit the key.
    #[serde(default, rename = "isItalic", skip_serializing_if = "is_false")]
    pub is_italic: bool,
    #[serde(default, rename = "linkStyle", skip_serializing_if = "Option::is_none")]
    pub link_style: Option<String>,
    /// If set, use the instanceInterpolations, otherwise calculate from axisValues.
    ///
    /// Always set to 1, otherwise omit the key.
    #[serde(
        default,
        rename = "manualInterpolation",
        skip_serializing_if = "is_false"
    )]
    pub manual_interpolation: bool,
    /// The style name
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "deserialize_export_type"
    )]
    pub export_type: ExportType,
    #[serde(default, rename = "userData", skip_serializing_if = "is_default")]
    pub user_data: Dictionary,
    #[serde(
        default,
        rename = "weightClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub weight_class: Option<Plist>, // String or integer
    #[serde(
        default,
        rename = "widthClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub width_class: Option<Plist>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Copy)]
pub enum ExportType {
    #[default]
    Static,
    #[serde(rename = "variable")]
    Variable,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Property {
    SingularProperty {
        key: SingularPropertyKey,
        value: String,
    },
    LocalizedProperty {
        key: LocalizedPropertyKey,
        values: Vec<LocalizedValue>,
    },
    // For properties that are not recognized. For example, there's a version of
    // glyphs that puts the `designer` property in the `properties` array with a
    // localized value.
    Junk(Plist),
}

impl Property {
    pub(crate) fn singular(key: SingularPropertyKey, value: String) -> Self {
        Property::SingularProperty { key, value }
    }
    pub(crate) fn localized_with_default(key: LocalizedPropertyKey, value: String) -> Self {
        Property::LocalizedProperty {
            key,
            values: vec![LocalizedValue {
                language: "dflt".to_string(),
                value,
            }],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum LocalizedPropertyKey {
    #[serde(rename = "familyNames")]
    FamilyNames,
    #[serde(rename = "copyrights")]
    Copyrights,
    #[serde(rename = "designers")]
    Designers,
    #[serde(rename = "manufacturers")]
    Manufacturers,
    #[serde(rename = "licenses")]
    Licenses,
    #[serde(rename = "trademarks")]
    Trademarks,
    #[serde(rename = "descriptions")]
    Descriptions,
    #[serde(rename = "sampleTexts")]
    SampleTexts,
    #[serde(rename = "compatibleFullNames")]
    CompatibleFullNames,
    #[serde(rename = "styleNames")]
    StyleNames,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SingularPropertyKey {
    #[serde(rename = "designer")]
    Designer,
    #[serde(rename = "manufacturer")]
    Manufacturer,
    #[serde(rename = "designerURL")]
    DesignerUrl,
    #[serde(rename = "manufacturerURL")]
    ManufacturerUrl,
    #[serde(rename = "licenseURL")]
    LicenseUrl,
    #[serde(rename = "postscriptFullName")]
    PostscriptFullName,
    #[serde(rename = "postscriptFontName")]
    PostscriptFontName,
    #[serde(rename = "WWSFamilyName")]
    WwsFamilyName,
    #[serde(rename = "versionString")]
    VersionString,
    #[serde(rename = "vendorID")]
    VendorID,
    #[serde(rename = "uniqueID")]
    UniqueID,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LocalizedValue {
    language: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stem {
    name: String,
    #[serde(default)]
    pub horizontal: bool,
}
