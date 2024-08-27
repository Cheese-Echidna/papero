use image::Rgb;
use crate::*;
use Direction::*;

pub struct Wave;

impl Generator for Wave {
    fn gen_image(args: &Args) -> DynamicImage {
        let sea = Rgb([10, 10, 255]);
        let coast = Rgb([255, 255, 10]);
        let land = Rgb([10, 255, 10]);

        /*
        Sea tiles can only go below or to the side of coast tiles, or anywhere next to other sea tiles
        Coast tiles can go to the side of land, sea, or other cost tiles, but only above sea tiles and below land ones.
        It makes no attempt to infer any more complex rules, like "sea tiles must be adjacent to at least one other sea tile"
        */

        let pallet: Vec<ColourRule> = vec![ColourRule::new(sea, vec![])];


        Image::blank(WIDTH, HEIGHT)
    }

    fn name() -> &'static str {
        "Wave Function Collapse - First"
    }
}

struct ColourRule {
    data: Color,
    rules: Vec<Rule>,
}

impl ColourRule {
    fn new(data: Color, rules: Vec<Rule>) -> Self {
        Self {
            data,
            rules,
        }
    }
}

struct Rule {
    dir: Direction,
    allows: Vec<Color>,
    must_have: Vec<Color>,
}

impl Rule {
    fn new(dir: Direction, allows: Vec<Color>, must_have: Vec<Color>) -> Self {
        Self {
            dir,
            allows,
            must_have,
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn to_tuple(&self) -> (usize, usize) {
        // (x,y)
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::UpLeft => (-1, 1),
            Direction::UpRight => (1, 1),
            Direction::DownLeft => (-1, -1),
            Direction::DownRight => (1, -1),
        }
    }
}

/*
LAND = GREEN
COAST = YELLOW
SEA = BLUE

*/