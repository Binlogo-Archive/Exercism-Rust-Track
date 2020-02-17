pub fn encode(source: &str) -> String {
    let mut encoder = Encoder::new();
    encoder.encode(source);
    encoder.encoded
}

pub fn decode(source: &str) -> String {
    let mut decoder = Decoder::new();
    decoder.decode(source);
    decoder.decoded
}

struct Encoder {
    encoded: String,
    peek_char: char,
    counter: u32,
}

impl Encoder {
    fn new() -> Self {
        Self {
            encoded: String::new(),
            peek_char: '\0',
            counter: 0,
        }
    }

    fn consume(&mut self, c: char) {
        if self.peek_char == c {
            self.counter += 1;
        } else {
            if self.peek_char != '\0' {
                if self.counter > 0 {
                    self.encoded.push_str(&(self.counter + 1).to_string());
                }
                self.encoded.push(self.peek_char);
            }
            self.peek_char = c;
            self.counter = 0;
        }
    }

    fn encode(&mut self, source: &str) -> &Self {
        source.chars().for_each(|c| self.consume(c));
        self.consume('\0');
        self
    }
}

struct Decoder {
    decoded: String,
    count: u32,
}

impl Decoder {
    fn new() -> Self {
        Self {
            decoded: String::new(),
            count: 0,
        }
    }

    fn consume(&mut self, c: char) {
        match c.to_digit(10) {
            Some(n) => self.count = self.count * 10 + n,
            None => {
                if self.count > 0 {
                    self.decoded
                        .push_str(&(c.to_string().repeat(self.count as usize)));
                } else {
                    self.decoded.push(c);
                }
                self.count = 0;
            }
        }
    }

    fn decode(&mut self, source: &str) -> &Self {
        source.chars().for_each(|c| self.consume(c));
        self
    }
}
