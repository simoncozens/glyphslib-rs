const ILLEGAL_CHARACTERS: [char; 45] = [
    '"', '*', '+', '/', ':', '<', '>', '?', '[', '\\', ']', '|', '\0', // null character
    '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\t', '\n', '\x0B', '\x0C',
    '\r', '\x0E', '\x0F', '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17', '\x18',
    '\x19', '\x1A', '\x1B', '\x1C', '\x1D', '\x1E', '\x1F', '\x7F',
];
const RESERVED_FILE_NAMES: [&str; 13] = [
    "con", "prn", "aux", "clock$", "nul", "com1", "com2", "com3", "com4", "lpt1", "lpt2", "lpt3",
    "a:-z:", // That one doesn't look right
];
const MAX_FILE_NAME_LENGTH: usize = 255;

pub(crate) fn user_name_to_file_name(name: &str) -> String {
    // replace an initial period with an _
    let mut user_name = name.to_string();
    if user_name.starts_with('.') {
        user_name.replace_range(0..1, "_");
    }
    // filter the user name (replace illegal characters with _, add _ to all non-lower characters)
    let mut filtered_username = String::new();
    for c in user_name.chars() {
        if ILLEGAL_CHARACTERS.contains(&c) {
            filtered_username.push('_');
        } else if c.to_lowercase().to_string() != c.to_string() {
            filtered_username.push(c);
            filtered_username.push('_');
        } else {
            filtered_username.push(c);
        }
    }
    // Clip to 255
    filtered_username.truncate(MAX_FILE_NAME_LENGTH);
    // Test for illegal file names
    let mut parts = vec![];
    for part in filtered_username.split('.') {
        if RESERVED_FILE_NAMES.contains(&part.to_lowercase().as_str()) {
            parts.push(format!("_{part}"));
        } else {
            parts.push(part.to_string());
        }
    }
    parts.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username() {
        assert_eq!(user_name_to_file_name("a"), "a");
        assert_eq!(user_name_to_file_name("A"), "A_");
        assert_eq!(user_name_to_file_name("AE"), "A_E_");
        assert_eq!(user_name_to_file_name("Ae"), "A_e");
        assert_eq!(user_name_to_file_name("ae"), "ae");
        assert_eq!(user_name_to_file_name("aE"), "aE_");
        assert_eq!(user_name_to_file_name("a.alt"), "a.alt");
        assert_eq!(user_name_to_file_name("A.alt"), "A_.alt");
        assert_eq!(user_name_to_file_name("A.Alt"), "A_.A_lt");
        assert_eq!(user_name_to_file_name("A.aLt"), "A_.aL_t");
        assert_eq!(user_name_to_file_name("A.alT"), "A_.alT_");
        assert_eq!(user_name_to_file_name("T_H"), "T__H_");
        assert_eq!(user_name_to_file_name("T_h"), "T__h");
        assert_eq!(user_name_to_file_name("t_h"), "t_h");
        assert_eq!(user_name_to_file_name("F_F_I"), "F__F__I_");
        assert_eq!(user_name_to_file_name("f_f_i"), "f_f_i");
        assert_eq!(user_name_to_file_name("Aacute_V.swash"), "A_acute_V_.swash");
        assert_eq!(user_name_to_file_name(".notdef"), "_notdef");
        assert_eq!(user_name_to_file_name("con"), "_con");
        assert_eq!(user_name_to_file_name("CON"), "C_O_N_");
        assert_eq!(user_name_to_file_name("con.alt"), "_con.alt");
        assert_eq!(user_name_to_file_name("alt.con"), "alt._con");
    }
}
