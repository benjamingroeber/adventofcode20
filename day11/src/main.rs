use helpers::read_file;
use itertools::Itertools;
use std::convert::TryFrom;
use std::error::Error;
use thiserror::Error;
use std::fmt::{Display, Formatter, Write};
use crate::Tile::SeatTaken;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day11.txt")?;
    let tiles = parse_grid(&input);

    let mut ferry = Ferry { grid: tiles? };
    let mut i = 0;
    loop {
        // println!("{}\n{}", ferry.grid, i);
        let result  = ferry.apply_rules_to_self();
        i += 1;

        if result == SeatingRules::NoChange {
            break
        }
    }
    let count_seats_occupied = ferry.grid.items.iter().filter(|s|**s==SeatTaken).count();
    println!("Occupied seats after {} iterations: {}", i, count_seats_occupied);

    Ok(())
}

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

    fn apply_rules(&self) -> impl Iterator<Item = Tile> + '_ {
        self.grid.items().map(move |((col, row), tile)| match tile {
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
            }
        })
    }

    fn apply_rules_to_self(&mut self) -> SeatingRules {
        let mut new_grid = self.grid.clone();
        for (i,tile) in self.apply_rules().enumerate() {
            new_grid.items[i] = tile
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
    type Error = GridError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::SeatTaken),
            'L' => Ok(Tile::SeatEmpty),
            // replace with option?
            '.' => Ok(Tile::Floor),
            _ => Err(GridError::UnknownTileError(value)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid<T> {
    columns: usize,
    items: Vec<T>,
}

impl<T> Grid<T> {
    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        // println!("{}", self.columns);
        if col < self.columns {
            self.items.get(row * self.columns + col)
        } else { None }
    }

    pub fn get_unchecked(&self, col: usize, row: usize) -> &T {
        if col < self.columns {
            &self.items[row * self.columns + col]
        } else {
            panic!("Index out of bounds!")
        }
    }

    pub fn items(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let rows = self.items.len() / self.columns;
        (0..rows)
            .cartesian_product(0..self.columns)
            .map(move |(row, col)| ((col, row), self.get_unchecked(col, row)))
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            if i % self.columns == 0 {
                writeln!(f, "")?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

fn parse_grid(s: &str) -> Result<Grid<Tile>, GridError> {
    let mut common_length = None;
    let mut tiles = Vec::with_capacity(s.len());
    for line in s.lines() {
        // sanity check for same length of all rows
        let line_length = line.chars().count();
        if let Some(size) = common_length {
            // println!("'{}' {} {}", line, size, line_length);
            if size != line_length {
                return Err(GridError::InconsistentXDimensionError);
            }
        } else {
            common_length = Some(line_length)
        }
        for tile in line.chars().map(Tile::try_from) {
            tiles.push(tile?)
        }
    }

    Ok(Grid {
        // if this is empty here, it means that there were no iterations on any lines before
        columns: common_length.unwrap_or(0),
        items: tiles,
    })
}

#[derive(Clone, Copy, Debug, Error)]
enum GridError {
    // #[error("could not parse Grid from string")]
    // ParseError,
    #[error("not all lines have the same length")]
    InconsistentXDimensionError,
    #[error("unknown tile {0}, only '#', '.' and 'L' are allowed")]
    UnknownTileError(char),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tile::{Floor, SeatEmpty, SeatTaken};

    #[test]
    fn test_parse_grid() {
        let input = "LLL\n###\n...";
        let expected = vec![
            SeatEmpty, SeatEmpty, SeatEmpty, SeatTaken, SeatTaken, SeatTaken, Floor, Floor, Floor,
        ];

        let grid = parse_grid(&input).unwrap();

        assert_eq!(grid.items, expected)
    }

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

    #[test]
    fn test_grid_items() {
        let input = "LLL\n###\n...";
        let grid = parse_grid(&input).unwrap();

        let items: Vec<_> = grid.items().map(|(_,tile)|*tile).collect();
        let newgrid = Grid{items, columns: grid.columns};
        println!("{}", grid);
        println!("{}", newgrid);
        assert_eq!(newgrid.items, grid.items)
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

        let ferry = Ferry{ grid };

        // all seats are empty
        assert!(ferry.grid.items().all(|(_,t)|*t==SeatEmpty||*t==Floor));
        // all seats are now filled
        assert!(ferry.apply_rules().all(|t|t==SeatTaken||t==Floor));
    }

}
