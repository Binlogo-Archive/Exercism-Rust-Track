use std::iter;
/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    atbash_iter(plain)
        .enumerate()
        .flat_map(|(index, c)| {
            let separate = index != 0 && index % 5 == 0;
            iter::once(' ')
                .filter(move |_| separate)
                .chain(iter::once(c))
        })
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash_iter(cipher).collect()
}

fn atbash_iter<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter_map(trans)
}

fn trans(c: char) -> Option<char> {
    let val = if c.is_ascii_digit() {
        c
    } else if c.is_ascii_alphabetic() {
        (b'z' - (c.to_ascii_lowercase() as u8 - b'a')) as char
    } else {
        return None;
    };
    Some(val)
}
