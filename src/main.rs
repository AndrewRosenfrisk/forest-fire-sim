use core::fmt;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::{thread_rng, Rng};
use std::{collections::HashMap, io::stdout, thread, time::Duration};

const WIDTH: u16 = 79;
const HEIGHT: u16 = 22;
const TREE_DENSITY: f64 = 0.2;
const GROW_CHANCE: f64 = 0.01;
const FIRE_CHANCE: f64 = 0.01;
const PAUSE: f32 = 0.5;

fn main() {
    let mut rng = thread_rng();
    let mut forest = new_forest();

    loop {
        execute!(
            stdout(),
            MoveTo(0, 0),
            Clear(All),
            Clear(Purge),
            DisableLineWrap,
            Hide
        )
        .unwrap();
        display_forest(&forest);

        let mut next = HashMap::new();

        for x in 0..=WIDTH {
            for y in 0..=HEIGHT {
                let point = &Point { x, y };
                if next.contains_key(point) {
                    continue;
                }

                let value = forest.get(point);

                if value.is_none() && rng.gen_bool(GROW_CHANCE) {
                    next.insert(*point, Value::TREE);
                } else if value.is_some()
                    && *value.unwrap() == Value::TREE
                    && rng.gen_bool(FIRE_CHANCE)
                {
                    next.insert(*point, Value::FIRE);
                } else if value.is_some() && *value.unwrap() == Value::FIRE {
                    let adjacent = &[-1, 0, 1];
                    for ax in adjacent {
                        for ay in adjacent {
                            let nx = (x as i32) + ax;
                            let ny = (y as i32) + ay;

                            if nx >= 0 && ny >= 0 {
                                let neighbor = &Point {
                                    x: nx as u16,
                                    y: ny as u16,
                                };

                                if forest.contains_key(neighbor)
                                    && *forest.get(neighbor).unwrap() == Value::TREE
                                {
                                    next.insert(*neighbor, Value::FIRE);
                                }
                            }
                        }
                    }
                } else if value.is_some() {
                    next.insert(*point, *value.unwrap());
                }
            }
        }
        forest = next;

        thread::sleep(Duration::from_secs_f32(PAUSE));
    }
}

fn new_forest() -> HashMap<Point, Value> {
    let mut rng = thread_rng();
    let mut forest: HashMap<Point, Value> = HashMap::new();
    for x in 0..=WIDTH {
        for y in 0..=HEIGHT {
            if rng.gen_bool(TREE_DENSITY) {
                forest.insert(Point { x, y }, Value::TREE);
            }
        }
    }
    forest
}

fn display_forest(forest: &HashMap<Point, Value>) {
    forest.iter().for_each(|(point, value)| {
        let color = match value {
            Value::TREE => Color::Green,
            Value::FIRE => Color::Red,
        };
        execute!(
            stdout(),
            MoveTo(point.x, point.y),
            SetForegroundColor(color),
            Print(value)
        )
        .unwrap();
    });

    execute!(
        stdout(),
        MoveTo(0, HEIGHT + 1),
        SetForegroundColor(Color::White)
    )
    .unwrap();
    println!("Grow chance: {:.0}%", (GROW_CHANCE * 100.0));
    println!("Lightning chance: {:.0}%", (FIRE_CHANCE * 100.0));
    println!("Press Ctrl-C to quit.");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(PartialEq, Clone, Copy)]
enum Value {
    TREE,
    FIRE,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::TREE => write!(f, "A"),
            Value::FIRE => write!(f, "W"),
        }
    }
}
