pub mod common;
pub mod glyphs2;
pub mod glyphs3;
mod serde;
mod upgrade;
mod utils;
use std::{ffi::OsStr, fs, path};

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

#[derive(Debug, Clone)]
pub enum Font {
    Glyphs2(Glyphs2),
    Glyphs3(Glyphs3),
}
impl Font {
    pub fn load(glyphs_file: &path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        if glyphs_file.extension() == Some(OsStr::new("glyphspackage")) {
            return Font::load_package(glyphs_file);
        }
        let raw_content = fs::read_to_string(glyphs_file)?;
        Self::load_str(&raw_content)
    }
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
    pub fn as_glyphs3(&self) -> Option<&Glyphs3> {
        match self {
            Font::Glyphs3(glyphs3) => Some(glyphs3),
            _ => None,
        }
    }
    pub fn as_glyphs2(&self) -> Option<&Glyphs2> {
        match self {
            Font::Glyphs2(glyphs2) => Some(glyphs2),
            _ => None,
        }
    }
    pub fn upgrade(&self) -> Self {
        match self {
            Font::Glyphs2(glyphs2) => Font::Glyphs3(Into::into(glyphs2.clone())),
            Font::Glyphs3(_) => self.clone(),
        }
    }

    pub fn upgrade_in_place(&mut self) {
        *self = self.upgrade();
    }

    pub fn to_string(&self) -> Result<String, openstep_plist::error::Error> {
        match self {
            Font::Glyphs2(glyphs2) => openstep_plist::ser::to_string(glyphs2),
            Font::Glyphs3(glyphs3) => openstep_plist::ser::to_string(glyphs3),
        }
    }

    pub fn save(&self, path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
        if path.extension() == Some(OsStr::new("glyphspackage")) {
            return self.save_package(path);
        }

        fs::write(path, self.to_string()?)?;
        Ok(())
    }

    fn load_package(glyphs_file: &path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let fontinfo_file = glyphs_file.join("fontinfo.plist");
        let raw_content = fs::read_to_string(fontinfo_file)?;
        let mut toplevel = Plist::parse(&raw_content)?.expect_dict()?;
        let ui_state_file = glyphs_file.join("UIState.plist");
        if let Ok(ui_state) = fs::read_to_string(ui_state_file) {
            let ui_state_plist = Plist::parse(&ui_state)?;
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
        let glyph_order_file = glyphs_file.join("order.plist");
        let glyph_order_plist = fs::read_to_string(glyph_order_file)?;
        let glyph_order = Plist::parse(&glyph_order_plist).and_then(|p| p.expect_array())?;
        let mut glyphs = vec![];
        for glyph in glyph_order.iter() {
            if let Some(name) = glyph.as_str() {
                let glyph_file = glyphs_file
                    .join("glyphs")
                    .join(format!("{}.glyph", user_name_to_file_name(name)));
                if let Ok(glyph_content) = fs::read_to_string(glyph_file) {
                    let glyph_plist = Plist::parse(&glyph_content)?;
                    glyphs.push(glyph_plist);
                }
            }
        }
        toplevel.insert("glyphs".into(), Plist::Array(glyphs));
        Self::from_plist(Plist::Dictionary(toplevel))
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
        assert_eq!(plist, new_plist);
    }
}
