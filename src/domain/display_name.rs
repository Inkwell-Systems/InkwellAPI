use unicode_segmentation::UnicodeSegmentation;

// Since the inner field is ... private (by definition),
// it can only be accessed from here.
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(s: String) -> DisplayName {
        if !is_valid_name(&s) {
            panic!("Name is invalid. Pure whitespace, a length > 256 or containing any of the characters '' is deemed illegal.")
        }

        DisplayName(s)
    }
}

impl AsRef<str> for DisplayName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for DisplayName {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

fn is_valid_name(name: &str) -> bool {
    let forbidden_chars = "/(){}[]|\"\\<>\'";

    let whitespace_check = name.trim().is_empty();
    let graphesme_check = name.graphemes(true).count() > 256;
    let forbidden_chars_check =
        name.chars().any(|c| forbidden_chars.contains(c));

    let f = whitespace_check || graphesme_check || forbidden_chars_check;
    !f
}
