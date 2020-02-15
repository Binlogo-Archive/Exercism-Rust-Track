pub fn encode(n: u64) -> String {
    Groups::new(n).encode()
}

#[derive(Debug)]
struct Groups(Vec<Group>);

impl Groups {
    fn new(val: u64) -> Self {
        let groups = {
            let mut val_string = val.to_string();
            while val_string.len() % 3 != 0 {
                val_string = format!("0{}", val_string);
            }
            let mut parsed_chunks = val_string
                .chars()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| chunk.into_iter().map(|c| *c).collect::<String>())
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            parsed_chunks.reverse();
            parsed_chunks.into_iter().map(|n| Group::new(n)).collect()
        };
        Groups(groups)
    }

    fn encode(&self) -> String {
        let mut res = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, group)| match group.encode() {
                None => None,
                Some(string) => match encode_scales(index) {
                    Some(scale) => Some(format!("{} {}", string, scale)),
                    None => Some(string),
                },
            })
            .collect::<Vec<_>>();
        print!("-=---{:?}", res);
        if res.len() == 0 {
            return "zero".to_owned();
        }
        res.reverse();
        res.join(" ")
    }
}

#[derive(Debug)]
struct Group {
    hundreds: Hundreds,
    tens: Tens,
}

impl Group {
    fn new(val: usize) -> Self {
        assert!(val < 1000);
        let hundreds = Hundreds::new(val / 100);
        let tens = Tens::new(val % 100);
        Group { hundreds, tens }
    }

    fn encode(&self) -> Option<String> {
        match (self.hundreds.encode(), self.tens.encode()) {
            (None, None) => None,
            (Some(hun), None) => Some(hun),
            (None, Some(ten)) => Some(ten),
            (Some(hun), Some(ten)) => Some(format!("{} {}", hun, ten)),
        }
    }
}

#[derive(Debug)]
struct Tens {
    tens: usize,
    ones: usize,
}

impl Tens {
    fn new(val: usize) -> Self {
        assert!(val < 100);
        let tens = val / 10;
        let ones = val % 10;
        Tens { tens, ones }
    }

    fn encode(&self) -> Option<String> {
        let val = self.tens * 10 + self.ones;
        if val == 0 {
            return None;
        } else if val < 10 {
            encode_ones(self.ones)
        } else if val < 20 {
            encode_teens(self.ones)
        } else {
            let tens = encode_ties(self.tens);
            if self.ones == 0 {
                return tens;
            }
            let ones = encode_ones(self.ones);
            match (tens, ones) {
                (Some(tens), Some(ones)) => Some(vec![tens, ones].join("-")),
                (Some(tens), None) => Some(tens),
                _ => None,
            }
        }
    }
}

#[derive(Debug)]
struct Hundreds(usize);

impl Hundreds {
    fn new(val: usize) -> Self {
        assert!(val < 10);
        Hundreds(val)
    }

    fn encode(&self) -> Option<String> {
        if self.0 == 0 {
            return None;
        }
        let hundreds = encode_ones(self.0);
        hundreds.map(|res| format!("{} hundred", res))
    }
}

fn encode_ones(n: usize) -> Option<String> {
    let res = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => return None,
    };
    Some(res.to_string())
}

fn encode_teens(n: usize) -> Option<String> {
    let res = match n {
        0 => "ten",
        1 => "eleven",
        2 => "twelve",
        3 => "thirteen",
        4 => "fourteen",
        5 => "fifteen",
        6 => "sixteen",
        7 => "seventeen",
        8 => "eighteen",
        9 => "nineteen",
        _ => return None,
    };
    Some(res.to_string())
}

fn encode_ties(n: usize) -> Option<String> {
    let res = match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => return None,
    };
    Some(res.to_string())
}

fn encode_scales(n: usize) -> Option<String> {
    let res = match n {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => return None,
    };
    Some(res.to_string())
}
