//! # glyphslib-rs
//!
//! A Rust library for reading, writing, and manipulating Glyphs font source files (.glyphs and .glyphspackage).
//!
//! This crate provides full support for both Glyphs 2 and Glyphs 3 file formats, allowing you to:
//! - Load and parse Glyphs files
//! - Access and modify font data (masters, instances, glyphs, layers, etc.)
//! - Serialize modified data back to Glyphs format
//! - Convert between Glyphs 2 and Glyphs 3 formats
//!
//! ## File Format Support
//!
//! - **Glyphs 2**: The original file format using OpenStep Property Lists
//! - **Glyphs 3**: The modern file format with enhanced features and better structure
//!
//! The library automatically detects which format is being used and provides a unified interface
//! through the [`Font`] enum.
//!
//! ## Examples
//!
//! ### Loading a Glyphs file
//!
//! ```no_run
//! use glyphslib::Font;
//! use std::path::Path;
//!
//! // Load a .glyphs or .glyphspackage file
//! let font = Font::load(Path::new("MyFont.glyphs")).unwrap();
//!
//! // Check the format version
//! match &font {
//!     Font::Glyphs2(g2) => println!("Loaded Glyphs 2 file: {}", g2.family_name),
//!     Font::Glyphs3(g3) => println!("Loaded Glyphs 3 file: {}", g3.family_name),
//! }
//! ```
//!
//! ### Examining font data
//!
//! ```no_run
//! use glyphslib::{Font, GlyphsFile};
//! use std::path::Path;
//!
//! let font = Font::load(Path::new("MyFont.glyphs")).unwrap();
//!
//! // Access common properties regardless of version
//! let font = font.as_glyphs3().unwrap();
//! println!("Family: {}", font.family_name());
//! println!("Number of glyphs: {}", font.glyphs().len());
//!
//! // Iterate through glyphs
//! for glyph in font.glyphs() {
//!     println!("Glyph: {}", glyph.name());
//! }
//!
//! // Access masters
//! for master in font.masters() {
//!     println!("Master: {}", master.name());
//! }
//! ```
//!
//! ### Modifying font data
//!
//! ```no_run
//! use glyphslib::Font;
//! use std::path::Path;
//! use glyphslib::glyphs3::{Property, SingularPropertyKey};
//!
//! let mut font = Font::load(Path::new("MyFont.glyphs")).unwrap();
//!
//! // Modify properties based on version
//! match &mut font {
//!     Font::Glyphs2(g2) => {
//!         g2.designer = Some("Jane Doe".to_string());
//!         // Modify a glyph
//!         if let Some(glyph) = g2.glyphs.iter_mut().find(|g| g.name == "A") {
//!             glyph.export = false;
//!         }
//!     }
//!     Font::Glyphs3(g3) => {
//!         g3.properties.push(Property::singular(SingularPropertyKey::Designer, "Jane Doe".to_string()));
//!         // Modify a glyph
//!         if let Some(glyph) = g3.glyphs.iter_mut().find(|g| g.name == "A") {
//!             glyph.export = false;
//!         }
//!     }
//! }
//! ```
//!
//! ### Serializing back to Glyphs format
//!
//! ```no_run
//! use glyphslib::Font;
//! use std::path::Path;
//! use std::fs;
//!
//! let font = Font::load(Path::new("MyFont.glyphs")).unwrap();
//!
//! // Serialize to string
//! let output = match &font {
//!     Font::Glyphs2(g2) => openstep_plist::to_string(g2).unwrap(),
//!     Font::Glyphs3(g3) => openstep_plist::to_string(g3).unwrap(),
//! };
//!
//! // Write to file
//! fs::write("MyFont_modified.glyphs", output).unwrap();
//! ```
//!
//! ### Converting between formats
//!
//! ```no_run
//! use glyphslib::Font;
//! use std::path::Path;
//!
//! let font = Font::load(Path::new("MyFont.glyphs")).unwrap();
//!
//! // Convert Glyphs 2 to Glyphs 3
//! let glyphs3 = font.upgrade();
//! ```

#![deny(missing_docs)]
/// Common types and structures shared between Glyphs 2 and Glyphs 3 formats
pub mod common;
/// Glyphs 2 file format structures
pub mod glyphs2;
/// Glyphs 3 file format structures
pub mod glyphs3;
mod serde;
mod traits;
mod upgrade;
mod utils;
use std::{collections::HashMap, ffi::OsStr, fs, path};

pub use traits::GlyphsFile;

use glyphs2::Glyphs2;
use glyphs3::Glyphs3;
pub use openstep_plist::Plist;
use openstep_plist::{de::Deserializer, Dictionary};

use utils::user_name_to_file_name;

fn is_glyphs3(plist: &Plist) -> bool {
    plist
        .as_dict()
        .map(|d| d.contains_key(".formatVersion"))
        .unwrap_or(false)
}

/// A font loaded from a Glyphs file, either version 2 or 3
///
/// This enum allows working with both Glyphs file format versions through a unified interface.
/// Use the [`GlyphsFile`] trait to access common properties regardless of version.
#[derive(Debug, Clone, PartialEq)]
pub enum Font {
    /// A Glyphs 2 format font
    Glyphs2(Glyphs2),
    /// A Glyphs 3 format font
    Glyphs3(Glyphs3),
}
impl Font {
    /// Load a Glyphs file from disk
    ///
    /// Supports both `.glyphs` files and `.glyphspackage` directories.
    /// The file format version is automatically detected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glyphslib::Font;
    /// use std::path::Path;
    ///
    /// let font = Font::load(Path::new("MyFont.glyphs")).unwrap();
    /// ```
    pub fn load(glyphs_file: &path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        if glyphs_file.extension() == Some(OsStr::new("glyphspackage")) {
            return Font::load_package(glyphs_file);
        }
        let raw_content = fs::read_to_string(glyphs_file)?;
        Self::load_str(&raw_content)
    }

    /// Load a Glyphs package from in-memory file entries.
    ///
    /// The map keys must be paths relative to the package root, for example:
    /// - `fontinfo.plist`
    /// - `order.plist`
    /// - `UIState.plist` (optional)
    /// - `glyphs/<glyph-file-name>.glyph`
    pub fn load_package_entries(
        entries: &HashMap<String, String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let normalized_entries: HashMap<String, String> = entries
            .iter()
            .map(|(path, contents)| {
                (
                    path.replace('\\', "/")
                        .trim_start_matches("./")
                        .trim_start_matches('/')
                        .to_string(),
                    contents.clone(),
                )
            })
            .collect();

        let raw_content = normalized_entries
            .get("fontinfo.plist")
            .ok_or("Missing fontinfo.plist in glyphspackage entries")?;

        let mut toplevel = Plist::parse(raw_content)?.expect_dict()?;

        if let Some(ui_state) = normalized_entries.get("UIState.plist") {
            let ui_state_plist = Plist::parse(ui_state)?;
            // UIState.plist contains a dictionary with a key "displayStrings".
            // However. the Glyphs3 non-package format has this key as "DisplayStrings" (with a capital 'D').
            // So we can't just merge dictionaries, we have to rewrite the key.
            toplevel.insert(
                "DisplayStrings".into(),
                ui_state_plist
                    .expect_dict()?
                    .get("displayStrings")
                    .cloned()
                    .unwrap_or(Plist::Array(vec![])),
            );
        }

        let glyph_order_plist = normalized_entries
            .get("order.plist")
            .ok_or("Missing order.plist in glyphspackage entries")?;
        let glyph_order = Plist::parse(glyph_order_plist).and_then(|p| p.expect_array())?;

        let mut glyphs = vec![];
        for glyph in glyph_order.iter() {
            if let Some(name) = glyph.as_str() {
                let glyph_path = format!("glyphs/{}.glyph", user_name_to_file_name(name));
                if let Some(glyph_content) = normalized_entries.get(&glyph_path) {
                    let glyph_plist = Plist::parse(glyph_content)?;
                    glyphs.push(glyph_plist);
                }
            }
        }

        toplevel.insert("glyphs".into(), Plist::Array(glyphs));
        Self::from_plist(Plist::Dictionary(toplevel))
    }

    /// Load a Glyphs file from a string
    ///
    /// Parses the OpenStep Property List format and deserializes into the appropriate
    /// Glyphs 2 or Glyphs 3 structure based on the format version marker.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use glyphslib::Font;
    ///
    /// let glyphs_data = std::fs::read_to_string("MyFont.glyphs").unwrap();
    /// let font = Font::load_str(&glyphs_data).unwrap();
    /// ```
    pub fn load_str(raw_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let plist = Plist::parse(raw_content)?;
        Font::from_plist(plist)
    }

    fn from_plist(plist: Plist) -> Result<Self, Box<dyn std::error::Error>> {
        let deserializer = &mut Deserializer::from_plist(&plist);
        if is_glyphs3(&plist) {
            let glyphs3: Glyphs3 = serde_path_to_error::deserialize(deserializer)?;
            Ok(Font::Glyphs3(glyphs3))
        } else {
            let glyphs2: Glyphs2 = serde_path_to_error::deserialize(deserializer)?;
            Ok(Font::Glyphs2(glyphs2))
        }
    }

    /// Get a reference to the font as a Glyphs 3 structure, if it is one
    ///
    /// Returns `None` if the font is in Glyphs 2 format.
    pub fn as_glyphs3(&self) -> Option<&Glyphs3> {
        match self {
            Font::Glyphs3(glyphs3) => Some(glyphs3),
            _ => None,
        }
    }

    /// Get a reference to the font as a Glyphs 2 structure, if it is one
    ///
    /// Returns `None` if the font is in Glyphs 3 format.
    pub fn as_glyphs2(&self) -> Option<&Glyphs2> {
        match self {
            Font::Glyphs2(glyphs2) => Some(glyphs2),
            _ => None,
        }
    }

    /// Returns a reference to font, ignoring the difference between Glyphs2 and Glyphs3.
    ///
    /// Sometimes you don't need to know which version of Glyphs you are dealing with,
    /// you just want to access the common functionality. The [GlyphsFile] trait
    /// provides an interface to methods which are common to both Glyphs2 and Glyphs3.
    pub fn font(&self) -> &dyn GlyphsFile {
        match self {
            Font::Glyphs2(glyphs2) => glyphs2,
            Font::Glyphs3(glyphs3) => glyphs3,
        }
    }

    /// Returns a mutable reference to font, ignoring the difference between Glyphs2 and Glyphs3.
    pub fn font_mut(&mut self) -> &mut dyn GlyphsFile {
        match self {
            Font::Glyphs2(glyphs2) => glyphs2,
            Font::Glyphs3(glyphs3) => glyphs3,
        }
    }

    /// Returns a Glyphs3 version of the font.
    pub fn upgrade(&self) -> Self {
        match self {
            Font::Glyphs2(glyphs2) => Font::Glyphs3(Into::into(glyphs2.clone())),
            Font::Glyphs3(_) => self.clone(),
        }
    }

    /// Turns a Glyphs2 font into a Glyphs3 font in place.
    pub fn upgrade_in_place(&mut self) {
        *self = self.upgrade();
    }

    /// Serializes the font to a a Plist in string format.
    pub fn to_string(&self) -> Result<String, openstep_plist::error::Error> {
        match self {
            Font::Glyphs2(glyphs2) => openstep_plist::ser::to_string(glyphs2),
            Font::Glyphs3(glyphs3) => openstep_plist::ser::to_string(glyphs3),
        }
    }

    /// Saves the font to a file.
    pub fn save(&self, path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
        if path.extension() == Some(OsStr::new("glyphspackage")) {
            return self.save_package(path);
        }

        fs::write(path, self.to_string()?)?;
        Ok(())
    }

    fn load_package(glyphs_file: &path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut entries = HashMap::new();

        entries.insert(
            "fontinfo.plist".to_string(),
            fs::read_to_string(glyphs_file.join("fontinfo.plist"))?,
        );

        if let Ok(ui_state) = fs::read_to_string(glyphs_file.join("UIState.plist")) {
            entries.insert("UIState.plist".to_string(), ui_state);
        }

        let glyph_order_plist = fs::read_to_string(glyphs_file.join("order.plist"))?;
        entries.insert("order.plist".to_string(), glyph_order_plist.clone());

        let glyph_order = Plist::parse(&glyph_order_plist).and_then(|p| p.expect_array())?;
        for glyph in glyph_order.iter() {
            if let Some(name) = glyph.as_str() {
                let glyph_file = glyphs_file
                    .join("glyphs")
                    .join(format!("{}.glyph", user_name_to_file_name(name)));
                if let Ok(glyph_content) = fs::read_to_string(glyph_file) {
                    entries.insert(
                        format!("glyphs/{}.glyph", user_name_to_file_name(name)),
                        glyph_content,
                    );
                }
            }
        }

        Self::load_package_entries(&entries)
    }

    fn save_package(&self, glyphs_file: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Font::Glyphs3(glyphs3) = self {
            let glyphs_dir = glyphs_file.join("glyphs");
            fs::create_dir_all(&glyphs_dir)?;
            let mut glyph_order: Vec<Plist> = vec![];
            for glyph in &glyphs3.glyphs {
                glyph_order.push(Plist::String(glyph.name.clone()));
                let name = user_name_to_file_name(&glyph.name);
                let glyph_file = glyphs_dir.join(format!("{name}.glyph"));
                fs::write(glyph_file, openstep_plist::ser::to_string(glyph)?)?;
            }
            let glyphorder_file = glyphs_file.join("order.plist");
            fs::write(
                glyphorder_file,
                openstep_plist::ser::to_string(&glyph_order)?.trim(),
            )?;
            if !glyphs3.display_strings.is_empty() {
                let mut dict = Dictionary::new();
                dict.insert(
                    "displayStrings".into(), // In a UIState.plist this has a lowercase 'd'
                    Plist::Array(
                        glyphs3
                            .display_strings
                            .iter()
                            .map(|s| Plist::String(s.to_string()))
                            .collect(),
                    ),
                );
                let ui_state = Plist::Dictionary(dict);
                fs::write(
                    glyphs_file.join("UIState.plist"),
                    openstep_plist::ser::to_string(&ui_state)?,
                )?;
            }
            // Drop the glyphs and UI state now we have saved them.
            let mut toplevel = glyphs3.clone();
            toplevel.glyphs.clear();
            toplevel.display_strings.clear();
            fs::write(
                glyphs_file.join("fontinfo.plist"),
                openstep_plist::ser::to_string(&toplevel)?,
            )?;
            Ok(())
        } else {
            Err("Saving Glyphs2 as package is not supported".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glyphs3::Shape;
    // use pretty_assertions::assert_eq;
    use rstest::rstest;
    #[rstest]
    fn test_load_upgrade(#[files("resources/*glyphs")] path: PathBuf) {
        let font = Font::load(&path).unwrap();
        if font.as_glyphs2().is_some() {
            let newfont = font.upgrade();
            let outdir = path::Path::new("resources/upgraded/");
            newfont
                .save(&outdir.join(path.file_name().unwrap()))
                .unwrap();
        }
    }

    #[test]
    fn test_component() {
        let file = "resources/RadioCanadaDisplay.glyphs";
        let font = Font::load(path::Path::new(file)).unwrap();
        let glyphs3 = font.as_glyphs3().unwrap();
        if let Shape::Component(component) = glyphs3
            .glyphs
            .iter()
            .find(|g| g.name == "eacute")
            .unwrap()
            .layers
            .first()
            .unwrap()
            .shapes
            .get(1)
            .unwrap()
        {
            assert_eq!(component.component_glyph, "acutecomb");
            assert_eq!(component.position, (152.0, 0.0));
        }
    }
    use path::PathBuf;

    #[rstest]
    fn test_roundtrip(
        #[files("resources/*glyphs")]
        #[exclude("RadioCanadaDisplay.glyphs")]
        path: PathBuf,
    ) {
        let raw_content = fs::read_to_string(path).unwrap();
        let plist = Plist::parse(&raw_content).unwrap();
        let font = Font::load_str(&raw_content).unwrap();
        let serialised = font.to_string().unwrap();
        let new_plist = Plist::parse(&serialised).unwrap();
        pretty_assertions::assert_eq!(plist, new_plist);
    }
}
