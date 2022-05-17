struct Part(Box<dyn Fn(&str) -> Option<Cmd>>);

impl Part {
    fn call(self, s: &str) -> Option<Cmd> {
        self.0(s)
    }
}

impl<T: Fn(&str) -> Option<Cmd> + 'static> From<T> for Part {
    fn from(f: T) -> Self {
        Self(Box::new(f))
    }
}

#[derive(Debug)]
pub enum Cmd {
    Remove(u16),
}

impl Cmd {
    fn from_char(c: char) -> Option<Part> {
        match c {
            'r' => Some(|s: &str| -> Option<Self> { Some(Self::Remove(s.parse().ok()?)) }.into()),
            _ => None,
        }
    }

    pub fn from_content(content: String) -> Option<Vec<Self>> {
        let mut args = content.trim().split_ascii_whitespace();

        if args.next()? == "s.w" {
            let mut subs = Vec::<Self>::new();

            while let Some(a) = args.next() {
                subs.push(
                    Self::from_char(a.split_once("-")?.1.chars().next()?)?.call(args.next()?)?,
                )
            }
            Some(subs)
        } else {
            None
        }
    }
}
