/*
#[derive(Debug)]
enum ParseGroupError {
    Regex,
    Number(ParseIntError),
}

impl From<ParseIntError> for ParseGroupError {
    fn from(err: ParseIntError) -> ParseGroupError {
        ParseBotError::Number(err)
    }
}

impl FromStr for Group {
    type Err = ParseBotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<n_units>\d+) units each with (?P<hp>\d+) hit points (?P<effects>\([\w ,;]+)\)? with an attack that does (?P<dmg>\d+) (?P<dtype>\w+) damage at initiative (?P<ini>\d+)"
            )
            .unwrap();
        }
        match RE.captures(s) {
            Some(parts) => {
                let n_units = parts.name("n_units").map_or("", |m| m.as_str()).parse::<i64>()?;
                let hp_each = parts.name("hp").map_or("", |m| m.as_str()).parse::<i64>()?;
                let damage = parts.name("dmg").map_or("", |m| m.as_str()).parse::<i64>()?;
                let dtype = parts.name("dtype").map_or("", |m| m.as_str())?;
                let initia

                Ok(Bot { x, y, z, r })
            }
            None => Err(ParseBotError::Regex),
        }
    }
}
*/
