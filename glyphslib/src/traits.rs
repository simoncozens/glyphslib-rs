use crate::common::Color;
use crate::common::FeatureClass;
use crate::glyphs2;
use crate::glyphs3;
// Utility traits to allow us to treat Glyphs2 and Glyphs3 structures interchangeablely
// where appropriate.
use crate::Glyphs2;
use crate::Glyphs3;
use paste::paste;

macro_rules! impl_glyphs_structure {
    (trait $trait_name:ident for $struct_name2:path, $struct_name3:path {$($inner:tt)*}) => {
        pub trait $trait_name {
            impl_glyphs_structure!(declarations $($inner)*);
        }

        impl $trait_name for $struct_name2 {
            impl_glyphs_structure!(implementations $($inner)*);
        }

        impl $trait_name for $struct_name3 {
            impl_glyphs_structure!(implementations $($inner)*);
        }
    };
    (declarations) => {};

    (declarations $field:ident ($documentation:tt) { $($accessors:tt)* }; $($tail:tt)*) => {
           impl_glyphs_structure!(declare accessor $field ($documentation) $($accessors)*);
           impl_glyphs_structure!(declarations $($tail)*);
    };

    (declare accessor $field:ident ($documentation:tt)) => {};
    (declare accessor $field:ident ($documentation:tt) get $type:ty; $($more_accessors:tt)*) => {
            #[doc = "Returns "]
            #[doc = $documentation]
            fn $field(&self) -> $type;
            impl_glyphs_structure!(declare accessor $field ($documentation) $($more_accessors)*);
    };
    (declare accessor $field:ident ($documentation:tt) get_ref $type:ty; $($more_accessors:tt)*) => {
            #[doc = "Returns "]
            #[doc = $documentation]
            fn $field(&self) -> $type;
            impl_glyphs_structure!(declare accessor $field ($documentation) $($more_accessors)*);
    };
    (declare accessor $field:ident ($documentation:tt) get_dyn $type:ty; $($more_accessors:tt)*) => {
            #[doc = "Returns a vector of trait objects of"]
            #[doc = $documentation]
            fn $field(&self) -> Vec<Box<$type>>;
            impl_glyphs_structure!(declare accessor $field ($documentation) $($more_accessors)*);
    };
    (declare accessor $field:ident ($documentation:tt) get_dyn_mut $type:ty; $($more_accessors:tt)*) => {
        paste! {
            #[doc = "Returns a mutable vector of trait objects of"]
            #[doc = $documentation]
            fn [<$field _mut>](&mut self) -> Vec<Box<$type>>;
            impl_glyphs_structure!(declare accessor $field ($documentation) $($more_accessors)*);
        }
    };
    (declare accessor $field:ident ($documentation:tt) get_mut $type:ty; $($more_accessors:tt)*) => {
        paste! {
            #[doc = "Returns a mutable reference to "]
            #[doc = $documentation]
            fn [<$field _mut>](&mut self) -> $type;
        }
        impl_glyphs_structure!(declare accessor $field ($documentation) $($more_accessors)*);
    };
    (declare accessor $field:ident ($documentation:tt) set $type:ty; $($more_accessors:tt)*) => {
        paste! {
            #[doc = "Sets "]
            #[doc = $documentation]
            fn [<set_ $field>](&mut self, value: $type);
        }
        impl_glyphs_structure!(declare accessor $field ($documentation) $($more_accessors)*);
    };

    (implementations) => {};
    (implementations $field:ident ($documentation:tt) { $($accessors:tt)* }; $($tail:tt)*) => {
           impl_glyphs_structure!(implement accessor $field ($documentation) $($accessors)*);
           impl_glyphs_structure!(implementations $($tail)*);
    };
    (implement accessor $field:ident ($documentation:tt)) => {};
    (implement accessor $field:ident ($documentation:tt) get $type:ty; $($more_accessors:tt)*) => {
        fn $field(&self) -> $type {
            &self.$field
        }
        impl_glyphs_structure!(implement accessor $field ($documentation) $($more_accessors)*);
    };
    (implement accessor $field:ident ($documentation:tt) get_ref $type:ty; $($more_accessors:tt)*) => {
        #[doc = "Returns "]
        #[doc = $documentation]
        fn $field(&self) -> $type {
            self.$field.as_deref()
        }
        impl_glyphs_structure!(implement accessor $field ($documentation) $($more_accessors)*);
    };

    (implement accessor $field:ident ($documentation:tt) get_dyn $type:ty; $($more_accessors:tt)*) => {
        fn $field(&self) -> Vec<Box<$type>> {
            self.$field.iter().map(|m| Box::new(m as $type)).collect::<Vec<_>>()
        }
        impl_glyphs_structure!(implement accessor $field ($documentation) $($more_accessors)*);
    };
    (implement accessor $field:ident ($documentation:tt) set $type:ty; $($more_accessors:tt)*) => {
        paste! {
            fn [<set_ $field>](&mut self, value: $type) {
                self.$field = value;
            }
        }
        impl_glyphs_structure!(implement accessor $field ($documentation) $($more_accessors)*);
    };
    (implement accessor $field:ident ($documentation:tt) get_mut $type:ty; $($more_accessors:tt)*) => {
        paste! {
            fn [<$field _mut>](&mut self) -> $type {
                &mut self.$field
            }
        }
        impl_glyphs_structure!(implement accessor $field ($documentation) $($more_accessors)*);
    };
    (implement accessor $field:ident ($documentation:tt) get_dyn_mut $type:ty; $($more_accessors:tt)*) => {
        paste! {
            fn [<$field _mut>](&mut self) -> Vec<Box<$type>> {
                self.$field.iter_mut().map(|m| Box::new(m as $type)).collect::<Vec<_>>()
            }
        }
        impl_glyphs_structure!(implement accessor $field ($documentation) $($more_accessors)*);
    };

}

impl_glyphs_structure!(trait GlyphsMaster for glyphs2::Master, glyphs3::Master {
    custom_parameters ("the master-specific custom parameters") {
        get &[crate::common::CustomParameter];
        get_mut &mut Vec<crate::common::CustomParameter>;
        set Vec<crate::common::CustomParameter>;
    };
});

impl_glyphs_structure!(trait GlyphsFile for Glyphs2, Glyphs3 {
    app_version ("the version of the Glyphs application that created this file") {
        get &str;
        set String;
    };
    date ("the date when the file was created or last modified") {
        get &str;
        set String;
    };
    display_strings ("the display strings used in the Glyphs UI") {
        get &[String];
        get_mut &mut Vec<String>;
        set Vec<String>;
    };
    classes ("the feature classes defined in the font") {
        get &[FeatureClass];
        get_mut &mut Vec<FeatureClass>;
        set Vec<FeatureClass>;
    };
    custom_parameters ("the font-global custom parameters") {
        get &[crate::common::CustomParameter];
        get_mut &mut Vec<crate::common::CustomParameter>;
        set Vec<crate::common::CustomParameter>;
    };
    family_name ("the font family name") {
        get &str;
        set String;
    };
    feature_prefixes ("the feature prefixes defined in the font") {
        get &[crate::common::FeaturePrefix];
        get_mut &mut Vec<crate::common::FeaturePrefix>;
        set Vec<crate::common::FeaturePrefix>;
    };
    features ("OpenType feature definitions") {
        get &[crate::common::Feature];
        get_mut &mut Vec<crate::common::Feature>;
        set Vec<crate::common::Feature>;
    };
    masters ("the masters defined in the font") {
        get_dyn &dyn GlyphsMaster;
        get_dyn_mut &mut dyn GlyphsMaster;
    };
    glyphs ("the glyphs defined in the font") {
        get_dyn &dyn GlyphsGlyph;
        get_dyn_mut &mut dyn GlyphsGlyph;
    };
});
impl_glyphs_structure!(trait GlyphsGlyph for glyphs2::Glyph, glyphs3::Glyph {
    kern_bottom ("the bottom kerning group of the glyph") {
        get_ref Option<&str>;
        set Option<String>;
    };
    name ("the name of the glyph") {
        get &str;
        set String;
    };
    unicode ("the Unicode code points assigned to the glyph") {
        get &[u32];
        get_mut &mut Vec<u32>;
        set Vec<u32>;
    };
    category ("the category of the glyph") {
        get_ref Option<&str>;
        set Option<String>;
    };
    color ("the color of the glyph") {
        get &Option<Color>;
        set Option<Color>;
    };
});
