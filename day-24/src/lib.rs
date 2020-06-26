use std::{collections::BTreeSet, convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

mod world;

use world::World;

pub struct Solution {
    world: World,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let world: World = read_file(input_file)?.parse()?;

        Ok(Self { world })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        24
    }

    fn solve(&self, part: Part) -> String {
        let mut world = self.world;

        match part {
            Part::One => {
                let mut existing = BTreeSet::new();

                while existing.insert(world) {
                    world = world.step();
                }

                format!(
                    "first world that repeats has biodiversity rating of: {}",
                    world.biodiversity()
                )
            }
            Part::Two => todo!(),
        }
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One]
    }
}
