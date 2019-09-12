use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    // input.chars().rev().collect::<String>()
    // input.chars().rev().collect()
    // UnicodeSegmentation::graphemes(input, true).rev().collect()
    input.graphemes(true).rev().collect::<String>()
}
