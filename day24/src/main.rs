#![allow(unused_doc_comments)]

use std::cmp::min;
use std::collections::HashMap;
use std::env;

#[allow(unused_assignments)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    if task == "battle" {
        // harcoding my input because it's easier than writing code to parse it
        let mut groups = input();
        if let Some((_, n_left)) = battle(&mut groups) {
            println!("{}", n_left);
        } else {
            println!("no winner");
        }
    } else if task == "boost" {
        let groups = input();

        // prior to binary search
        // first find an upper boost that lets immune win
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

fn input() -> Vec<Group> {
    vec![
        Group {
            id: 0,
            side: Side::Immune,
            n_units: 4592,
            hp_each: 2061,
            immunities: vec![Attack::Slashing, Attack::Radiation],
            weaknesses: vec![Attack::Cold],
            attack: Attack::Fire,
            damage: 4,
            initiative: 9,
        },
        Group {
            id: 1,
            side: Side::Immune,
            n_units: 1383,
            hp_each: 3687,
            immunities: vec![],
            weaknesses: vec![],
            attack: Attack::Radiation,
            damage: 26,
            initiative: 15,
        },
        Group {
            id: 2,
            side: Side::Immune,
            n_units: 2736,
            hp_each: 6429,
            immunities: vec![Attack::Slashing],
            weaknesses: vec![],
            attack: Attack::Slashing,
            damage: 20,
            initiative: 2,
        },
        Group {
            id: 3,
            side: Side::Immune,
            n_units: 777,
            hp_each: 3708,
            immunities: vec![Attack::Radiation, Attack::Cold],
            weaknesses: vec![Attack::Slashing, Attack::Fire],
            attack: Attack::Cold,
            damage: 39,
            initiative: 4,
        },
        Group {
            id: 4,
            side: Side::Immune,
            n_units: 6761,
            hp_each: 2792,
            immunities: vec![
                Attack::Bludgeoning,
                Attack::Fire,
                Attack::Slashing,
                Attack::Cold,
            ],
            weaknesses: vec![],
            attack: Attack::Radiation,
            damage: 3,
            initiative: 17,
        },
        Group {
            id: 5,
            side: Side::Immune,
            n_units: 6028,
            hp_each: 5537,
            immunities: vec![Attack::Slashing],
            weaknesses: vec![],
            attack: Attack::Radiation,
            damage: 7,
            initiative: 6,
        },
        Group {
            id: 6,
            side: Side::Immune,
            n_units: 2412,
            hp_each: 2787,
            immunities: vec![],
            weaknesses: vec![],
            attack: Attack::Bludgeoning,
            damage: 9,
            initiative: 20,
        },
        Group {
            id: 7,
            side: Side::Immune,
            n_units: 6042,
            hp_each: 7747,
            immunities: vec![Attack::Radiation],
            weaknesses: vec![],
            attack: Attack::Slashing,
            damage: 12,
            initiative: 12,
        },
        Group {
            id: 8,
            side: Side::Immune,
            n_units: 1734,
            hp_each: 7697,
            immunities: vec![],
            weaknesses: vec![Attack::Radiation, Attack::Cold],
            attack: Attack::Cold,
            damage: 38,
            initiative: 10,
        },
        Group {
            id: 9,
            side: Side::Immune,
            n_units: 4391,
            hp_each: 3250,
            immunities: vec![],
            weaknesses: vec![],
            attack: Attack::Cold,
            damage: 7,
            initiative: 19,
        },
        Group {
            id: 10,
            side: Side::Infection,
            n_units: 820,
            hp_each: 46229,
            immunities: vec![Attack::Cold, Attack::Bludgeoning],
            weaknesses: vec![],
            attack: Attack::Slashing,
            damage: 106,
            initiative: 18,
        },
        Group {
            id: 11,
            side: Side::Infection,
            n_units: 723,
            hp_each: 30757,
            immunities: vec![],
            weaknesses: vec![Attack::Bludgeoning],
            attack: Attack::Fire,
            damage: 80,
            initiative: 3,
        },
        Group {
            id: 12,
            side: Side::Infection,
            n_units: 2907,
            hp_each: 51667,
            immunities: vec![Attack::Bludgeoning],
            weaknesses: vec![Attack::Slashing],
            attack: Attack::Fire,
            damage: 32,
            initiative: 1,
        },
        Group {
            id: 13,
            side: Side::Infection,
            n_units: 2755,
            hp_each: 49292,
            immunities: vec![],
            weaknesses: vec![Attack::Bludgeoning],
            attack: Attack::Fire,
            damage: 34,
            initiative: 5,
        },
        Group {
            id: 14,
            side: Side::Infection,
            n_units: 5824,
            hp_each: 24708,
            immunities: vec![
                Attack::Bludgeoning,
                Attack::Cold,
                Attack::Radiation,
                Attack::Slashing,
            ],
            weaknesses: vec![],
            attack: Attack::Bludgeoning,
            damage: 7,
            initiative: 11,
        },
        Group {
            id: 15,
            side: Side::Infection,
            n_units: 7501,
            hp_each: 6943,
            immunities: vec![Attack::Slashing],
            weaknesses: vec![Attack::Cold],
            attack: Attack::Radiation,
            damage: 1,
            initiative: 8,
        },
        Group {
            id: 16,
            side: Side::Infection,
            n_units: 573,
            hp_each: 10367,
            immunities: vec![],
            weaknesses: vec![Attack::Slashing, Attack::Cold],
            attack: Attack::Radiation,
            damage: 30,
            initiative: 16,
        },
        Group {
            id: 17,
            side: Side::Infection,
            n_units: 84,
            hp_each: 31020,
            immunities: vec![],
            weaknesses: vec![Attack::Cold],
            attack: Attack::Slashing,
            damage: 639,
            initiative: 14,
        },
        Group {
            id: 18,
            side: Side::Infection,
            n_units: 2063,
            hp_each: 31223,
            immunities: vec![Attack::Bludgeoning],
            weaknesses: vec![Attack::Radiation],
            attack: Attack::Cold,
            damage: 25,
            initiative: 13,
        },
        Group {
            id: 19,
            side: Side::Infection,
            n_units: 214,
            hp_each: 31088,
            immunities: vec![],
            weaknesses: vec![Attack::Fire],
            attack: Attack::Slashing,
            damage: 271,
            initiative: 7,
        },
    ]
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
    //let mut i = 0;
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
