#![allow(unused_doc_comments)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;

use regex::Regex;

#[allow(unused_assignments)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let mut groups: Vec<Group> = Vec::new();
    let mut side: Option<Side> = None;
    for line in fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|l| !l.trim().is_empty())
    {
        if line.starts_with("Immune System") {
            side = Some(Side::Immune);
        } else if line.starts_with("Infection") {
            side = Some(Side::Infection)
        } else if let Some(s) = side {
            let id = groups.len();
            groups.push(group_from_str(line.trim(), id, s).expect("Couldn't parse group"));
        } else {
            panic!("Couldn't parse the input file");
        }
    }

    if task == "battle" {
        if let Some((_, n_left)) = battle(&mut groups) {
            println!("{}", n_left);
        } else {
            println!("no winner");
        }
    } else if task == "boost" {
        let mut boost = 1;
        let mut n_left = 0;
        loop {
            let mut boosted_groups = boost_groups(&groups, boost);
            if let Some((side, n_side)) = battle(&mut boosted_groups) {
                if side == Side::Immune {
                    n_left = n_side;
                    break;
                }
            }
            boost += 1;
        }
        println!("took {} boost", boost);
        println!("{}", n_left);
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug)]
enum ParseGroupError {
    Regex,
    Attack,
    Number(ParseIntError),
}

impl From<ParseIntError> for ParseGroupError {
    fn from(err: ParseIntError) -> ParseGroupError {
        ParseGroupError::Number(err)
    }
}

fn attack_from_str(s: &str) -> Result<Attack, ParseGroupError> {
    match s.trim() {
        "fire" => Ok(Attack::Fire),
        "cold" => Ok(Attack::Cold),
        "slashing" => Ok(Attack::Slashing),
        "bludgeoning" => Ok(Attack::Bludgeoning),
        "radiation" => Ok(Attack::Radiation),
        _ => Err(ParseGroupError::Attack),
    }
}

fn weak_immune_from_str(s: &str) -> Result<(Vec<Attack>, Vec<Attack>), ParseGroupError> {
    lazy_static! {
        static ref WEAK_RE: Regex = Regex::new(r"weak to (?P<weak>[\w ,]+)").unwrap();
        static ref IMMUNE_RE: Regex = Regex::new(r"immune to (?P<immune>[\w ,]+)").unwrap();
    }
    let weaknesses: Vec<Attack> = match WEAK_RE.captures(s) {
        Some(parts) => parts
            .name("weak")
            .map_or("", |m| m.as_str())
            .split(',')
            .map(attack_from_str)
            .collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };
    let immunities: Vec<Attack> = match IMMUNE_RE.captures(s) {
        Some(parts) => parts
            .name("immune")
            .map_or("", |m| m.as_str())
            .split(',')
            .map(attack_from_str)
            .collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };
    Ok((weaknesses, immunities))
}

fn group_from_str(s: &str, id: usize, side: Side) -> Result<Group, ParseGroupError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<n_units>\d+) units each with (?P<hp>\d+) hit points(?P<effects> \([\w ,;]+\))? with an attack that does (?P<dmg>\d+) (?P<atype>\w+) damage at initiative (?P<ini>\d+)"
        )
        .unwrap();
    }
    match RE.captures(s) {
        Some(parts) => {
            let n_units = parts
                .name("n_units")
                .map_or("", |m| m.as_str())
                .parse::<i64>()?;
            let hp_each = parts.name("hp").map_or("", |m| m.as_str()).parse::<i64>()?;
            let damage = parts
                .name("dmg")
                .map_or("", |m| m.as_str())
                .parse::<i64>()?;
            let attack = attack_from_str(parts.name("atype").map_or("", |m| m.as_str()))?;
            let initiative = parts
                .name("ini")
                .map_or("", |m| m.as_str())
                .parse::<i64>()?;
            let (weaknesses, immunities) =
                weak_immune_from_str(parts.name("effects").map_or("", |m| m.as_str()))?;
            Ok(Group {
                id,
                side,
                n_units,
                hp_each,
                immunities,
                weaknesses,
                attack,
                damage,
                initiative,
            })
        }
        None => Err(ParseGroupError::Regex),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Attack {
    Slashing,
    Radiation,
    Cold,
    Fire,
    Bludgeoning,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Side {
    Immune,
    Infection,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Group {
    id: usize,
    side: Side,
    n_units: i64,
    hp_each: i64,
    immunities: Vec<Attack>,
    weaknesses: Vec<Attack>,
    attack: Attack,
    damage: i64,
    initiative: i64,
}

impl Group {
    fn effective_power(&self) -> i64 {
        self.n_units * self.damage
    }

    fn damage_to(&self, other: &Group) -> i64 {
        if other.immunities.contains(&self.attack) {
            0
        } else if other.weaknesses.contains(&self.attack) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn attack(&self, other: &mut Group) {
        let damage = self.damage_to(&other);
        let n_killed = min(damage / other.hp_each, other.n_units);
        other.n_units -= n_killed;
    }

    fn select_target(&self, others: &[Group], already_chosen: &[&usize]) -> Option<usize> {
        others
            .iter()
            .filter(|o| !already_chosen.contains(&&o.id))
            .filter(|o| o.side != self.side)
            .filter(|o| o.id != self.id)
            .filter(|o| o.n_units > 0)
            .max_by_key(|o| (self.damage_to(o), o.effective_power(), o.initiative))
            .filter(|o| self.damage_to(o) > 0)
            .map(|o| o.id)
    }
}

fn battle_step(groups: &mut [Group]) {
    let mut target_order = groups
        .iter()
        .map(|g| (g.effective_power(), g.initiative, g.id))
        .collect::<Vec<_>>();
    target_order.sort_by_key(|(eff_pow, ini, _)| (-eff_pow, -ini));
    let mut targets: HashMap<usize, usize> = HashMap::new();
    for (_, _, group_id) in target_order {
        let group = &groups[group_id];
        if let Some(target_id) =
            group.select_target(&groups, &targets.values().collect::<Vec<_>>()[..])
        {
            targets.insert(group_id, target_id);
        }
    }

    let mut attack_order = groups
        .iter()
        .map(|g| (g.initiative, g.id))
        .collect::<Vec<_>>();
    attack_order.sort_by_key(|(ini, _)| -ini);
    for (_, group_id) in attack_order {
        let group = &groups[group_id].clone();
        if let Some(target_id) = targets.get(&group_id) {
            group.attack(&mut groups[*target_id]);
        }
    }
}

fn battle(groups: &mut [Group]) -> Option<(Side, i64)> {
    let mut n_immune = 0;
    let mut n_infection = 0;
    loop {
        battle_step(groups);
        let new_n_immune: i64 = groups
            .iter()
            .filter(|g| g.side == Side::Immune)
            .map(|g| g.n_units)
            .sum();
        let new_n_infection: i64 = groups
            .iter()
            .filter(|g| g.side == Side::Infection)
            .map(|g| g.n_units)
            .sum();
        if n_immune == new_n_immune && n_infection == new_n_infection {
            return None;
        }
        n_immune = new_n_immune;
        n_infection = new_n_infection;
        if n_immune <= 0 || n_infection <= 0 {
            break;
        }
    }
    if n_immune > 0 {
        Some((Side::Immune, n_immune))
    } else if n_infection > 0 {
        Some((Side::Infection, n_infection))
    } else {
        None
    }
}

fn boost_groups(groups: &[Group], boost: i64) -> Vec<Group> {
    let mut boosted_groups = groups.to_vec();
    for ref mut group in &mut boosted_groups {
        if group.side == Side::Immune {
            group.damage += boost;
        }
    }
    boosted_groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle() {
        let mut groups = vec![
            Group {
                id: 0,
                side: Side::Immune,
                n_units: 17,
                hp_each: 5390,
                immunities: vec![],
                weaknesses: vec![Attack::Radiation, Attack::Bludgeoning],
                attack: Attack::Fire,
                damage: 4507,
                initiative: 2,
            },
            Group {
                id: 1,
                side: Side::Immune,
                n_units: 989,
                hp_each: 1274,
                immunities: vec![Attack::Fire],
                weaknesses: vec![Attack::Bludgeoning, Attack::Slashing],
                attack: Attack::Slashing,
                damage: 25,
                initiative: 3,
            },
            Group {
                id: 2,
                side: Side::Infection,
                n_units: 801,
                hp_each: 4706,
                immunities: vec![],
                weaknesses: vec![Attack::Radiation],
                attack: Attack::Bludgeoning,
                damage: 116,
                initiative: 1,
            },
            Group {
                id: 3,
                side: Side::Infection,
                n_units: 4485,
                hp_each: 2961,
                immunities: vec![Attack::Cold],
                weaknesses: vec![Attack::Fire, Attack::Cold],
                attack: Attack::Slashing,
                damage: 12,
                initiative: 4,
            },
        ];
        assert_eq!(Some((Side::Infection, 5216)), battle(&mut groups));
    }
    #[test]
    fn test_battle_boost() {
        let groups = vec![
            Group {
                id: 0,
                side: Side::Immune,
                n_units: 17,
                hp_each: 5390,
                immunities: vec![],
                weaknesses: vec![Attack::Radiation, Attack::Bludgeoning],
                attack: Attack::Fire,
                damage: 4507,
                initiative: 2,
            },
            Group {
                id: 1,
                side: Side::Immune,
                n_units: 989,
                hp_each: 1274,
                immunities: vec![Attack::Fire],
                weaknesses: vec![Attack::Bludgeoning, Attack::Slashing],
                attack: Attack::Slashing,
                damage: 25,
                initiative: 3,
            },
            Group {
                id: 2,
                side: Side::Infection,
                n_units: 801,
                hp_each: 4706,
                immunities: vec![],
                weaknesses: vec![Attack::Radiation],
                attack: Attack::Bludgeoning,
                damage: 116,
                initiative: 1,
            },
            Group {
                id: 3,
                side: Side::Infection,
                n_units: 4485,
                hp_each: 2961,
                immunities: vec![Attack::Cold],
                weaknesses: vec![Attack::Fire, Attack::Cold],
                attack: Attack::Slashing,
                damage: 12,
                initiative: 4,
            },
        ];
        let mut boosted_groups = boost_groups(&groups, 1570);
        assert_eq!(Some((Side::Immune, 51)), battle(&mut boosted_groups));
    }
}
