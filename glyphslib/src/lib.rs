pub mod common;
pub mod glyphs2;
pub mod glyphs3;
mod upgrade;
use std::{fs, path};

use glyphs2::Glyphs2;
use glyphs3::Glyphs3;
use openstep_plist::{de::Deserializer, Plist};

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
        let raw_content = fs::read_to_string(glyphs_file)?;
        Self::load_str(&raw_content)
    }
    pub fn load_str(raw_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let plist = Plist::parse(raw_content).unwrap();
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
        fs::write(path, self.to_string()?)?;
        Ok(())
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
