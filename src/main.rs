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
use std::{
    collections::HashMap,
    io::stdout,
    ops::{Add, Mul},
    thread,
    time::Duration,
};
const WIDTH: u16 = 79;
const HEIGHT: u16 = 22;
const TREE_DENSITY: f64 = 0.2;
const GROW_CHANCE: f64 = 0.01;
const FIRE_CHANCE: f64 = 0.01;
const EMPTY: &str = " ";
const TREE: &str = "A";
const FIRE: &str = "W";
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

        let mut next = Forest {
            width: forest.width,
            height: forest.height,
            forest: HashMap::new(),
        };

        for x in 0..=forest.width {
            for y in 0..=forest.height {
                let point = &Point { x, y };
                if next.forest.contains_key(point) {
                    continue;
                }

                if forest.forest.contains_key(point) {
                    let value = forest.forest.get(point).unwrap();

                    if value == EMPTY && rng.gen_bool(GROW_CHANCE) {
                        next.forest.insert(*point, TREE.to_string());
                    } else if value == TREE && rng.gen_bool(FIRE_CHANCE) {
                        next.forest.insert(*point, FIRE.to_string());
                    } else if value == FIRE {
                        let a: i32 = -1;
                        for ix in a..=1 {
                            for iy in a..=1 {
                                let nx = (x as i32).add(ix);
                                let ny = (y as i32).add(iy);
                                if nx >= 0 && ny >= 0 {
                                    let neighbor = &Point {
                                        x: nx as u16,
                                        y: ny as u16,
                                    };

                                    if forest.forest.contains_key(neighbor)
                                        && forest.forest.get(neighbor).unwrap() == TREE
                                    {
                                        next.forest.insert(*neighbor, FIRE.to_string());
                                    }
                                }
                            }
                        }
                        next.forest.insert(*point, EMPTY.to_string());
                    } else {
                        next.forest.insert(*point, value.to_string());
                    }
                }
            }
        }
        forest = next;

        thread::sleep(Duration::from_secs_f32(PAUSE));
    }
}

fn new_forest() -> Forest {
    let mut rng = thread_rng();
    let mut forest: HashMap<Point, String> = HashMap::new();
    for x in 0..=WIDTH {
        for y in 0..=HEIGHT {
            if rng.gen_bool(TREE_DENSITY) {
                forest.insert(Point { x, y }, TREE.to_string());
            } else {
                forest.insert(Point { x, y }, EMPTY.to_string());
            }
        }
    }
    Forest {
        width: WIDTH,
        height: HEIGHT,
        forest,
    }
}

fn display_forest(forest: &Forest) {
    for y in 0..=forest.height {
        for x in 0..=forest.width {
            let value = forest.forest.get(&Point { x, y }).unwrap();
            let color = match value.as_str() {
                TREE => Color::Green,
                FIRE => Color::Red,
                _ => Color::White,
            };

            execute!(
                stdout(),
                MoveTo(x, y),
                SetForegroundColor(color),
                Print(value)
            )
            .unwrap();
        }
    }
    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    println!("\nGrow chance: {:.0}%", (GROW_CHANCE.mul(100.0)));
    println!("Lightning chance: {:.0}%", (FIRE_CHANCE.mul(100.0)));
    println!("Press Ctrl-C to quit.");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

struct Forest {
    width: u16,
    height: u16,
    forest: HashMap<Point, String>,
}
