// Serde extensions for Glyphs data structures.

use itertools::Itertools;
use std::fmt;

use serde::{
    de::Visitor,
    ser::{SerializeSeq, SerializeStruct, SerializeTuple},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_with::SerializeAs;

use crate::glyphs3::{self, MetricType};
use crate::{
    common::NodeType,
    glyphs2::{self, AlignmentZone, CropRect},
};

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
pub(crate) fn is_default<T>(v: &T) -> bool
where
    T: Default + PartialEq,
{
    *v == T::default()
}

pub(crate) fn is_one_hundred(value: &i32) -> bool {
    *value == 100
}

pub(crate) fn one_hundred() -> i32 {
    100
}

impl<'de> Deserialize<'de> for MetricType {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant = String::deserialize(de)?;
        Ok(match variant.as_str() {
            "ascender" => MetricType::Ascender,
            "cap height" => MetricType::CapHeight,
            "slant height" => MetricType::SlantHeight,
            "x-height" => MetricType::XHeight,
            "midHeight" => MetricType::MidHeight,
            "topHeight" => MetricType::TopHeight,
            "bodyHeight" => MetricType::BodyHeight,
            "descender" => MetricType::Descender,
            "baseline" => MetricType::Baseline,
            "italic angle" => MetricType::ItalicAngle,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "unknown metric type: {variant}",
                )))
            }
        })
    }
}

struct SimpleNodeVisitor;
impl<'de> Visitor<'de> for SimpleNodeVisitor {
    type Value = glyphs3::Node;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple of 3 or 4 elements")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let x = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        let node_type = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
        let user_data = seq.next_element()?;
        Ok(glyphs3::Node {
            x,
            y,
            node_type,
            user_data,
        })
    }
}

impl<'de> Deserialize<'de> for glyphs3::Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SimpleNodeVisitor)
    }
}

impl Serialize for glyphs3::Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(3)?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.serialize_element(&self.node_type)?;
        if let Some(user_data) = &self.user_data {
            seq.serialize_element(user_data)?;
        }
        seq.end()
    }
}

impl Serialize for glyphs2::Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // string X Y (full node type) (SMOOTH)?
        let node_type = match self.node_type {
            NodeType::Line => "LINE",
            NodeType::Curve => "CURVE",
            NodeType::QCurve => "QCURVE",
            NodeType::OffCurve => "OFFCURVE",
            NodeType::LineSmooth => "LINE SMOOTH",
            NodeType::CurveSmooth => "CURVE SMOOTH",
            NodeType::QCurveSmooth => "QCURVE SMOOTH",
        };
        let node_str = format!("{} {} {}", self.x, self.y, node_type);
        let mut seq = serializer.serialize_seq(None)?;
        seq.serialize_element(&node_str)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for glyphs2::Node {
    fn deserialize<D>(deserializer: D) -> Result<glyphs2::Node, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NodeVisitor)
    }
}

#[derive(Debug, Default, Clone)]
struct NodeVisitor;

impl Visitor<'_> for NodeVisitor {
    type Value = glyphs2::Node;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with node data")
    }

    fn visit_str<E>(self, value: &str) -> Result<glyphs2::Node, E>
    where
        E: serde::de::Error,
    {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(E::custom("not enough parts"));
        }
        let x = parts[0]
            .parse::<f32>()
            .map_err(|_| E::custom("could not parse x"))?;
        let y = parts[1]
            .parse::<f32>()
            .map_err(|_| E::custom("could not parse y"))?;
        let smooth = parts.len() > 3 && parts[3] == "SMOOTH";
        let node_type = match (parts[2], smooth) {
            ("LINE", false) => NodeType::Line,
            ("CURVE", false) => NodeType::Curve,
            ("QCURVE", false) => NodeType::QCurve,
            ("OFFCURVE", false) => NodeType::OffCurve,
            ("LINE", true) => NodeType::LineSmooth,
            ("CURVE", true) => NodeType::CurveSmooth,
            ("QCURVE", true) => NodeType::QCurveSmooth,
            _ => return Err(E::custom("unknown node type")),
        };
        Ok(glyphs2::Node { x, y, node_type })
    }
}

impl Serialize for glyphs3::Layer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = 24; // Nobody's actually counting.
        let mut seq = serializer.serialize_struct("Layer", fields)?;
        if !self.anchors.is_empty() {
            seq.serialize_field("anchors", &self.anchors)?;
        }
        if !self.annotations.is_empty() {
            seq.serialize_field("annotations", &self.annotations)?;
        }
        if let Some(master_id) = &self.associated_master_id {
            seq.serialize_field("associatedMasterId", master_id)?;
        }
        if !self.attr.is_empty() {
            seq.serialize_field("attr", &self.attr)?;
        }
        if let Some(background) = &self.background {
            seq.serialize_field("background", background)?;
        }
        if let Some(background_image) = &self.background_image {
            seq.serialize_field("backgroundImage", background_image)?;
        }
        if let Some(color) = &self.color {
            seq.serialize_field("color", color)?;
        }
        if !self.guides.is_empty() {
            seq.serialize_field("guides", &self.guides)?;
        }
        if !self.hints.is_empty() {
            seq.serialize_field("hints", &self.hints)?;
        }
        if !self.layer_id.is_empty() {
            seq.serialize_field("layerId", &self.layer_id)?;
        }
        if let Some(metric_bottom) = &self.metric_bottom {
            seq.serialize_field("metricBottom", metric_bottom)?;
        }
        if let Some(metric_left) = &self.metric_left {
            seq.serialize_field("metricLeft", metric_left)?;
        }
        if let Some(metric_right) = &self.metric_right {
            seq.serialize_field("metricRight", metric_right)?;
        }
        if let Some(metric_top) = &self.metric_top {
            seq.serialize_field("metricTop", metric_top)?;
        }
        if let Some(metric_vert_width) = &self.metric_vert_width {
            seq.serialize_field("metricVertWidth", metric_vert_width)?;
        }
        if let Some(metric_width) = &self.metric_width {
            seq.serialize_field("metricWidth", metric_width)?;
        }
        if let Some(name) = &self.name {
            seq.serialize_field("name", name)?;
        }
        if !self.part_selection.is_empty() {
            seq.serialize_field("partSelection", &self.part_selection)?;
        }
        if !self.shapes.is_empty() {
            seq.serialize_field("shapes", &self.shapes)?;
        }
        if !self.user_data.is_empty() {
            seq.serialize_field("userData", &self.user_data)?;
        }
        if let Some(vert_origin) = &self.vert_origin {
            seq.serialize_field("vertOrigin", vert_origin)?;
        }
        if let Some(vert_width) = &self.vert_width {
            seq.serialize_field("vertWidth", vert_width)?;
        }
        if !self.visible {
            seq.serialize_field("visible", &self.visible)?;
        }
        if self.width != 0.0 || !self.layer_id.is_empty() {
            seq.serialize_field("width", &self.width)?;
        }
        seq.end()
    }
}

pub(crate) fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: i8 = Deserialize::deserialize(deserializer)?;
    Ok(s == 1)
}

pub(crate) fn anything_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(AnythingToBoolVisitor)
}

struct AnythingToBoolVisitor;

impl<'de> Visitor<'de> for AnythingToBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("anything")
    }

    fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(true)
    }
    fn visit_i64<E>(self, _v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(true)
    }
}

pub(crate) struct SerializeAsTuple<U> {
    _marker: std::marker::PhantomData<U>,
}

impl<T> SerializeAs<Vec<T>> for SerializeAsTuple<T>
where
    T: Serialize,
{
    fn serialize_as<S>(source: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match source.len() {
            1 => source
                .iter()
                .next()
                .expect("Cannot be empty")
                .serialize(serializer),
            n => {
                let mut arr = serializer.serialize_tuple(n)?;
                for item in source {
                    arr.serialize_element(&item)?;
                }
                arr.end()
            }
        }
    }
}

pub fn serialize_comma_hexstring<S>(value: &[u32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if value.is_empty() {
        return serializer.serialize_none();
    }
    if value.len() == 1 {
        return serializer.serialize_str(&format!("{:04X}", value[0]));
    }
    let mut seq = serializer.serialize_seq(None)?;
    for (ix, i) in value.iter().enumerate() {
        seq.serialize_element(&format!("{i:04X}"))?;
        if ix < value.len() - 1 {
            seq.serialize_element(",")?;
        }
    }
    seq.end()
}

pub fn deserialize_comma_hexstring<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(CommaHexStringVisitor)
}

struct CommaHexStringVisitor;

impl<'de> Visitor<'de> for CommaHexStringVisitor {
    type Value = Vec<u32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Unicode value")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Vec<u32>, E>
    where
        E: serde::de::Error,
    {
        // If the value is a single integer - it isn't! It's a hex string
        let s = format!("{value:04X}");

        Ok(vec![
            u32::from_str_radix(&s, 16).map_err(serde::de::Error::custom)?
        ])
    }

    fn visit_i64<E>(self, value: i64) -> Result<Vec<u32>, E>
    where
        E: serde::de::Error,
    {
        // If the value is a single integer - it isn't! It's a hex string
        let s = format!("{value:04X}");

        Ok(vec![
            u32::from_str_radix(&s, 16).map_err(serde::de::Error::custom)?
        ])
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let codepoints = v.split(',');
        let mut result = Vec::new();
        for codepoint in codepoints {
            result.push(u32::from_str_radix(codepoint, 16).map_err(serde::de::Error::custom)?);
        }
        Ok(result)
    }
}

// Well, this is going to get interesting.
pub(crate) trait CurlyBraceReceiver<T> {
    fn try_from_parts(parts: impl Iterator<Item = T>) -> Result<Self, String>
    where
        Self: Sized;
}

impl CurlyBraceReceiver<f32> for (f32, f32) {
    fn try_from_parts(parts: impl Iterator<Item = f32>) -> Result<Self, String> {
        let mut iter = parts.into_iter();
        let first = iter.next().ok_or_else(|| "Expected 2 parts".to_string())?;
        let second = iter.next().ok_or_else(|| "Expected 2 parts".to_string())?;
        Ok((first, second))
    }
}

pub(crate) struct CurlyBraceVisitor<T>
where
    T: CurlyBraceReceiver<f32>, // Maybe there's an argument for being EVEN MORE GENERIC but I think we're quite generic enough
{
    pub(crate) _marker: std::marker::PhantomData<T>,
}

impl<T> Default for CurlyBraceVisitor<T>
where
    T: CurlyBraceReceiver<f32>,
{
    fn default() -> Self {
        CurlyBraceVisitor {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Visitor<'_> for CurlyBraceVisitor<T>
where
    T: CurlyBraceReceiver<f32>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with curly braces (e.g. \"{800, 15}\")")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
        E: serde::de::Error,
    {
        let parts = value.trim_matches(|c| c == '{' || c == '}').split(',');
        let parts_vec: Vec<f32> = parts
            .map(|s| {
                s.trim()
                    .parse::<f32>()
                    .map_err(|e| E::custom(format!("failed to parse '{s}' as f32: {e}")))
            })
            .collect::<Result<Vec<_>, _>>()?;
        T::try_from_parts(parts_vec.into_iter())
            .map_err(|e| E::custom(format!("failed to parse parts: {e}")))
    }

    // It's usually a string! But just occasionally, we get a raw number
    // (for example in instanceInterpolations, if there is only one axis)
    fn visit_f64<E>(self, value: f64) -> Result<T, E>
    where
        E: serde::de::Error,
    {
        T::try_from_parts(std::iter::once(value as f32)).map_err(|e| E::custom(e))
    }
    fn visit_i64<E>(self, value: i64) -> Result<T, E>
    where
        E: serde::de::Error,
    {
        T::try_from_parts(std::iter::once(value as f32)).map_err(|e| E::custom(e))
    }
}

// We need our own IntoIterator so we can implement it on various builtins
pub(crate) trait MyIntoIterator<'a> {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
impl MyIntoIterator<'_> for &(f32, f32) {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1].into_iter()
    }
}
impl<'a> MyIntoIterator<'a> for &'a Vec<f32> {
    type Item = f32;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, f32>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::IntoIterator::into_iter(self).copied()
    }
}

pub(crate) fn serialize_commify<'a, S, T>(value: &'a T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: CurlyBraceReceiver<f32> + 'a,
    &'a T: MyIntoIterator<'a, Item = f32>,
{
    let middle: String = value.into_iter().map(|x| x.to_string()).join(", ");
    serializer.serialize_str(&format!("{{{middle}}}"))
}

pub(crate) fn deserialize_commify<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: CurlyBraceReceiver<f32>,
{
    deserializer.deserialize_any(CurlyBraceVisitor::<T>::default())
}

// So complicated our nice generic solution above doesn't work
pub(crate) trait CropRectReceiver {
    fn new(top: i32, left: i32, bottom: i32, right: i32) -> Self;
}
pub(crate) struct CropRectVisitor<T: CropRectReceiver> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for CropRectVisitor<T>
where
    T: CropRectReceiver,
{
    fn default() -> Self {
        CropRectVisitor {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<T> Visitor<'_> for CropRectVisitor<T>
where
    T: CropRectReceiver,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a crop rectangle (e.g. \"{{1,2},{3,4}}\")")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
        E: serde::de::Error,
    {
        let chunks = value
            .chars()
            .chunk_by(|&element| element != '{' && element != '}' && element != ',');
        let mut number_groups = chunks
            .into_iter()
            .filter(|(k, _v)| *k)
            .map(|(_k, v)| v.collect::<String>());
        let top = number_groups
            .next()
            .ok_or_else(|| E::custom("missing top"))?
            .parse::<i32>()
            .map_err(|_| E::custom("top not a number"))?;
        let left = number_groups
            .next()
            .ok_or_else(|| E::custom("missing left"))?
            .parse::<i32>()
            .map_err(|_| E::custom("left not a number"))?;
        let bottom = number_groups
            .next()
            .ok_or_else(|| E::custom("missing bottom"))?
            .parse::<i32>()
            .map_err(|_| E::custom("bottom not a number"))?;
        let right = number_groups
            .next()
            .ok_or_else(|| E::custom("missing right"))?
            .parse::<i32>()
            .map_err(|_| E::custom("right not a number"))?;
        Ok(T::new(top, left, bottom, right))
    }
}

impl<'de> Deserialize<'de> for glyphs2::Transform {
    fn deserialize<D>(deserializer: D) -> Result<glyphs2::Transform, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CurlyBraceVisitor::<glyphs2::Transform>::default())
    }
}
impl Serialize for glyphs2::Transform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_commify(self, serializer)
    }
}
impl MyIntoIterator<'_> for &glyphs2::Transform {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 6>;

    fn into_iter(self) -> Self::IntoIter {
        [self.m11, self.m12, self.m21, self.m22, self.t_x, self.t_y].into_iter()
    }
}

impl CurlyBraceReceiver<f32> for glyphs2::Transform {
    fn try_from_parts(parts: impl Iterator<Item = f32>) -> Result<Self, String> {
        let parts: Vec<f32> = parts.into_iter().collect();
        if parts.len() != 6 {
            return Err(format!(
                "Expected exactly 6 parts for glyphs2::Transform, got {}",
                parts.len()
            ));
        }
        Ok(glyphs2::Transform {
            m11: parts[0],
            m12: parts[1],
            m21: parts[2],
            m22: parts[3],
            t_x: parts[4],
            t_y: parts[5],
        })
    }
}

impl IntoIterator for &glyphs2::Transform {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 6>;

    fn into_iter(self) -> Self::IntoIter {
        [self.m11, self.m12, self.m21, self.m22, self.t_x, self.t_y].into_iter()
    }
}

impl<'de> Deserialize<'de> for CropRect {
    fn deserialize<D>(deserializer: D) -> Result<CropRect, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CropRectVisitor::<CropRect>::default())
    }
}

impl CropRectReceiver for CropRect {
    fn new(top: i32, left: i32, bottom: i32, right: i32) -> Self {
        CropRect {
            top,
            left,
            bottom,
            right,
        }
    }
}
impl Serialize for CropRect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        seq.serialize_element(&format!(
            "{{{{{},{}}},{{{}, {}}}}}",
            self.top, self.left, self.bottom, self.right
        ))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for AlignmentZone {
    fn deserialize<D>(deserializer: D) -> Result<AlignmentZone, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CurlyBraceVisitor::<AlignmentZone>::default())
    }
}
impl Serialize for AlignmentZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_commify(self, serializer)
    }
}
impl MyIntoIterator<'_> for &AlignmentZone {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.position, self.overshoot].into_iter()
    }
}

impl CurlyBraceReceiver<f32> for AlignmentZone {
    fn try_from_parts(parts: impl Iterator<Item = f32>) -> Result<Self, String> {
        let mut iter = parts.into_iter();
        let position = iter.next().ok_or_else(|| "Expected 2 parts".to_string())?;
        let overshoot = iter.next().ok_or_else(|| "Expected 2 parts".to_string())?;
        Ok(AlignmentZone {
            position,
            overshoot,
        })
    }
}

impl IntoIterator for &AlignmentZone {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.position, self.overshoot].into_iter()
    }
}

pub(crate) fn deserialize_export_type<'de, D>(
    deserializer: D,
) -> Result<glyphs3::ExportType, D::Error>
where
    D: Deserializer<'de>,
{
    let variant = String::deserialize(deserializer)?;
    Ok(match variant.as_str() {
        "static" => glyphs3::ExportType::Static,
        "variable" => glyphs3::ExportType::Variable,
        _ => {
            return Err(serde::de::Error::custom(format!(
                "unknown export type: {variant}"
            )))
        }
    })
}

impl CurlyBraceReceiver<f32> for Vec<f32> {
    fn try_from_parts(parts: impl Iterator<Item = f32>) -> Result<Self, String> {
        Ok(parts.collect())
    }
}
