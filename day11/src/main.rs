use helpers::{read_file, Grid};
use itertools::Itertools;
use std::convert::TryFrom;
use std::error::Error;
use thiserror::Error;
use std::fmt::{Display, Formatter, Write};
use crate::Tile::{SeatTaken, Floor};

// TODO Refactor second part
fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day11.txt")?;
    let tiles = parse_grid(&input);

    let mut ferry = Ferry { grid: tiles? };
    let mut i = 0;
    loop {
        let result  = ferry.apply_rules_to_self();
        i += 1;

        if result == SeatingRules::NoChange {
            break
        }
    }
    let count_seats_occupied = ferry.grid.iter().filter(|s|**s==SeatTaken).count();
    println!("Part1: Occupied seats after {} iterations: {}", i, count_seats_occupied);
    //
    let tiles = parse_grid(&input);
    let mut ferry = Ferry { grid: tiles? };
    // let result = run_until_stable(&ferry, Box::new(Ferry::apply_rules_part2) as Box<dyn Iterator<Item=_>>);
    let mut i = 0;
    loop {
        let result  = ferry.apply_rules2_to_self();
        i += 1;

        if result == SeatingRules::NoChange {
            break
        }
    }
    let count_seats_occupied = ferry.grid.iter().filter(|s|**s==SeatTaken).count();
    println!("Part2: Occupied seats after {} iterations: {}", i, count_seats_occupied);

    Ok(())
}

/*fn run_until_stable<F>(ferry: &Ferry, f:F) ->  Ferry
    where F: Fn(&Ferry) -> dyn Iterator<Item=((usize, usize), Tile)> {
    let mut init = ferry.clone();
    loop {
        let mut new = init.grid.clone();

        let iter = &f(ferry);
        for ((col, row), tile) in iter.iter() {
            new.set(col, row, tile)
        }
        if init.grid == new {
            return init
        } else {
            init.grid = new;
        }
    }
}*/

#[derive(Clone,Debug, Eq, PartialEq)]
struct Ferry {
    grid: Grid<Tile>,
}

impl Ferry {
    fn get_tile(&self, col: usize, row: usize) -> Option<Tile> {
        self.grid.get(col, row).copied()
    }

    fn neighbours(&self, col: usize, row: usize) -> impl Iterator<Item = Tile> + '_ {
        let start_col = start_idx(col);
        let start_row = start_idx(row);
        (start_row..=row + 1)
            .cartesian_product(start_col..=col + 1)
            .filter_map(move |(n_row, n_col)| {
                if n_col == col && n_row == row {
                    // Ignore self
                    None
                } else {
                    self.get_tile(n_col, n_row)
                }
            })
    }

    fn visible_seats(&self, col: usize, row: usize) -> Vec<Tile> {
        let mut tiles = Vec::new();
        // same row, horizontal left
        let mut i = 1;
        while i <= col {
            let tile = self.get_tile(col-(i), row);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // same column, vertical up
        let mut i = 1;
        while i <= row {
            let tile = self.get_tile(col, row-i);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // diagonal up left
        let mut i = 1;
        // println!("AAAA {} {} {}", row, col, i);
        while i <= row && i <= col {
            let tile = self.get_tile(col-i, row-i);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // diagonal up right
        let mut i = 1;
        while i <= row && col+i < self.grid.num_columns() {
            let tile = self.get_tile(col+i, row-i);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // same row horizontal right
        let mut i = 1;
        while col+i < self.grid.num_columns() {
            let tile = self.get_tile(col+i, row);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // same column vertical down
        let mut i = 1;
        while row+i < self.grid.num_rows() {
            let tile = self.get_tile(col, row+i);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // diagonal down right
        let mut i = 1;
        while col+i < self.grid.num_columns() && row+i < self.grid.num_rows() {
            let tile = self.get_tile(col+i, row+i);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }

        // diagonal down left
        let mut i = 1;
        while i <= col && row+i < self.grid.num_rows() {
            let tile = self.get_tile(col-i, row+i);
            if let Some(tile) = tile {
                if tile != Floor {
                    tiles.push(tile);
                    break
                }
            }
            i+=1
        }
        tiles
    }

    fn apply_rules(&self) -> impl Iterator<Item = ((usize, usize),Tile)> + '_ {
        self.grid.items_iter().map(move |((col, row), tile)| { let tile = match tile {
            Tile::Floor => *tile,
            Tile::SeatEmpty => {
                if ! self.neighbours(col, row).any(|n| n == Tile::SeatTaken) {
                    Tile::SeatTaken
                } else {
                    *tile
                }
            }
            Tile::SeatTaken => {
                if self.neighbours(col, row).filter(|n| *n == Tile::SeatTaken).count() >= 4 {
                    Tile::SeatEmpty
                } else {
                    *tile
                }
            }};
            ((col, row), tile)
        })
    }

    fn apply_rules_part2(&self) -> impl Iterator<Item = ((usize, usize),Tile)> + '_ {
        self.grid.items_iter().map(move |((col, row), tile)| { let tile = match tile {
            Tile::Floor => *tile,
            Tile::SeatEmpty => {
                if ! self.visible_seats(col, row).iter().any(|n| *n == Tile::SeatTaken) {
                    Tile::SeatTaken
                } else {
                    *tile
                }
            }
            Tile::SeatTaken => {
                if self.visible_seats(col, row).iter().filter(|n| **n == Tile::SeatTaken).count() >= 5 {
                    Tile::SeatEmpty
                } else {
                    *tile
                }
            }};
            ((col, row), tile)
        })
    }

    fn apply_rules_to_self(&mut self) -> SeatingRules {
        let mut new_grid = self.grid.clone();
        for ((col, row),tile) in self.apply_rules() {
            new_grid.set(col, row, tile)
        }
        if self.grid == new_grid {
            SeatingRules::NoChange
        } else {
            self.grid = new_grid;
            SeatingRules::Applied
        }
    }

    fn apply_rules2_to_self(&mut self) -> SeatingRules {
        let mut new_grid = self.grid.clone();
        for ((col, row),tile) in self.apply_rules_part2() {
            new_grid.set(col, row, tile)
        }
        if self.grid == new_grid {
            SeatingRules::NoChange
        } else {
            self.grid = new_grid;
            SeatingRules::Applied
        }
    }
}

fn start_idx(idx: usize) -> usize {
    if idx < 1 { idx } else { idx - 1 }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SeatingRules {
    Applied,
    NoChange
}

// TODO Refactor to Grid<Option<Seat>> with Seat::Empty and Seat::Occupied
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    SeatEmpty,
    SeatTaken,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Floor => '.',
            Tile::SeatEmpty => 'L',
            Tile::SeatTaken => '#',
        };
        f.write_char(c)
    }
}

impl TryFrom<char> for Tile {
    type Error = FerryError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::SeatTaken),
            'L' => Ok(Tile::SeatEmpty),
            // replace with option?
            '.' => Ok(Tile::Floor),
            _ => Err(FerryError::UnknownTile(value)),
        }
    }
}

fn parse_grid(s: &str) -> Result<Grid<Tile>, FerryError> {
    let mut common_length = None;
    let mut items = Vec::with_capacity(s.len());
    for line in s.lines() {
        // sanity check for same length of all rows
        let line_length = line.chars().count();
        if let Some(size) = common_length {
            if size != line_length {
                return Err(FerryError::InconsistentXDimension);
            }
        } else {
            common_length = Some(line_length)
        }
        for tile in line.chars().map(Tile::try_from) {
            items.push(tile?)
        }
    }

    // if common_length is empty here, it means that there were no iterations on any lines before
    Ok(Grid::with_items(items, common_length.unwrap_or(0))?)
}

#[derive(Clone, Copy, Debug, Error)]
enum FerryError {
    // #[error("could not parse Grid from string")]
    // ParseError,
    #[error("grid error")]
    Grid(#[from] helpers::GridError),
    #[error("not all lines have the same length")]
    InconsistentXDimension,
    #[error("unknown tile {0}, only '#', '.' and 'L' are allowed")]
    UnknownTile(char),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tile::{Floor, SeatEmpty, SeatTaken};

    // TODO move to helpers
    #[test]
    fn test_parse_grid() {
        let input = "LLL\n###\n...";
        let expected = vec![
            SeatEmpty, SeatEmpty, SeatEmpty, SeatTaken, SeatTaken, SeatTaken, Floor, Floor, Floor,
        ];

        let grid = parse_grid(&input).unwrap();

        assert_eq!(grid.items(), expected)
    }

    // TODO move to helpers
    #[test]
    fn test_grid_get() {
        let input = "LLL\n###\n...";
        let grid = parse_grid(&input).unwrap();

        assert_eq!(Some(&SeatEmpty), grid.get(0, 0));
        assert_eq!(Some(&SeatEmpty), grid.get(1, 0));
        assert_eq!(Some(&SeatEmpty), grid.get(2, 0));

        assert_eq!(Some(&SeatTaken), grid.get(0, 1));
        assert_eq!(Some(&SeatTaken), grid.get(1, 1));
        assert_eq!(Some(&SeatTaken), grid.get(2, 1));

        assert_eq!(Some(&Floor), grid.get(0, 2));
        assert_eq!(Some(&Floor), grid.get(1, 2));
        assert_eq!(Some(&Floor), grid.get(2, 2));
    }

    // TODO move to helpers
    #[test]
    fn test_grid_items() {
        let input = "LLL\n###\n...";
        let grid = parse_grid(&input).unwrap();

        let items: Vec<_> = grid.items_iter().map(|(_,tile)|*tile).collect();
        let new_grid = Grid::with_items(items, grid.num_columns()).unwrap();

        assert_eq!(new_grid.items(), grid.items())
    }

    #[test]
    fn test_ferry_get() {
        let input = "LLL\n###\n...";
        let ferry = Ferry {
            grid: parse_grid(&input).unwrap(),
        };

        assert_eq!(Some(SeatEmpty), ferry.get_tile(0, 0));
        assert_eq!(Some(SeatEmpty), ferry.get_tile(1, 0));
        assert_eq!(Some(SeatEmpty), ferry.get_tile(2, 0));

        assert_eq!(Some(SeatTaken), ferry.get_tile(0, 1));
        assert_eq!(Some(SeatTaken), ferry.get_tile(1, 1));
        assert_eq!(Some(SeatTaken), ferry.get_tile(2, 1));

        assert_eq!(Some(Floor), ferry.get_tile(0, 2));
        assert_eq!(Some(Floor), ferry.get_tile(1, 2));
        assert_eq!(Some(Floor), ferry.get_tile(2, 2));
    }

    #[test]
    fn test_ferry_get_neighbours() {
        let input = "LLL\n###\n...";

        let ferry = Ferry {
            grid: parse_grid(&input).unwrap(),
        };

        let neighbours_origin: Vec<_> = ferry.neighbours(0, 0).collect();
        let neighbours_middle: Vec<_> = ferry.neighbours(1, 1).collect();
        println!("START");
        let neighbours_bottom_right : Vec<_> = ferry.neighbours(2, 2).collect();
        println!("END");

        assert_eq!(&neighbours_origin, &vec![SeatEmpty, SeatTaken, SeatTaken]);
        assert_eq!(
            &neighbours_middle,
            &vec![SeatEmpty, SeatEmpty, SeatEmpty, SeatTaken, SeatTaken, Floor, Floor, Floor]
        );
        assert_eq!(&neighbours_bottom_right, &vec![SeatTaken, SeatTaken, Floor]);

    }
    
    #[test]
    fn test_part1_example() {
        let input = read_file("../assets/days/day11_example.txt").unwrap();
        let grid = parse_grid(&input).unwrap();

        let mut ferry = Ferry{ grid };

        // all seats are empty
        assert!(ferry.grid.items_iter().all(|(_,t)|*t==SeatEmpty||*t==Floor));
        // all seats are now filled
        assert!(ferry.apply_rules().all(|(_,t)|t==SeatTaken||t==Floor));

        // after 5 rounds there are no more changes
        for _ in 0..5 {
            ferry.apply_rules_to_self();
        }

        let expected = ferry.clone();
        ferry.apply_rules_to_self();
        assert_eq!(expected, ferry)
    }

    // Now, instead of considering just the eight immediately adjacent seats, consider the first
    // seat in each of those eight directions. For example, the empty seat below would see
    // eight occupied seats:
    // .......#.
    // ...#.....
    // .#.......
    // .........
    // ..#L....#
    // ....#....
    // .........
    // #........
    // ...#.....
    #[test]
    fn test_part2_visibility_example() {
        let input = read_file("../assets/days/day11_example_part2.txt").unwrap();
        let grid = parse_grid(&input).unwrap();

        let ferry = Ferry{ grid };

        assert_eq!(ferry.visible_seats(3,4).iter().filter(|t|**t==SeatTaken).count(), 8 );
    }

}
