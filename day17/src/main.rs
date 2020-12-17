use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;
use helpers::read_file;
use std::fmt::{Display, Formatter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day17.txt")?;
    let mut grid = parse_starting_state(&input)?;

    (0..6).for_each(|_|grid.step());

    let count = grid.coordinates.values().filter(|c|**c == Cube::Active).count();
    println!("{}", count);
    Ok(())
}

type Unit = isize;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

// x, y, z
type Coordinate = (Unit, Unit, Unit);
#[derive(Debug, Clone)]
struct Grid {
    coordinates: HashMap<Coordinate, Cube>,
}

impl Grid {
    fn get_cube_at(&self, c: Coordinate) -> Cube {
        *self.coordinates.get(&(c.0,c.1,c.2)).unwrap_or(&Cube::Inactive)
    }
    fn step(&mut self) {
        let (min, max) = self.get_coordinate_min_max();
        println!("{:?}, {:?}", min, max);
        let mut new = HashMap::new();
        for z in min.2-2..=max.2+2 {
            for y in min.1-2..=max.1+2 {
                for x in min.0-2..=max.0+2 {
                    let active_neighbours = self.get_neighbours((x,y,z)).filter(|c|*c == Cube::Active).count();
                    let cube = self.get_cube_at((x,y,z));
                    match cube {
                        Cube::Active => match active_neighbours {
                            2|3 => {new.insert((x,y,z), Cube::Active);},
                            _ => {}
                        },
                        Cube::Inactive => match active_neighbours {
                            3 => {new.insert((x,y,z), Cube::Active);},
                            _ => {}
                        }
                    }
                }
            }
        }
        self.coordinates = new;
    }

    fn get_coordinate_min_max(&self) -> (Coordinate, Coordinate) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_z = 0;

        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;

        for ((x,y,z),_) in self.coordinates.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *z < min_z {
                min_z = *z;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
            if *z > max_z {
                max_z = *z;
            }
        }
        ((min_x,min_y,min_z), (max_x, max_y, max_z))
    }

    fn get_neighbours(&self, cordinate: Coordinate) -> impl Iterator<Item = Cube> + '_ {
        (-1..=1).map(move |dz| {
            (-1..=1).map(move |dy| {
                (-1..=1).filter_map(move |dx| {
                    if dz == 0 && dy == 0 && dx == 0 {
                        None
                    } else {
                        Some(self.coordinates
                            .get(&(cordinate.0 + dx, cordinate.1 + dy, cordinate.2 + dz))
                            .copied()
                            .unwrap_or(Cube::Inactive))
                    }
                })
            }).flatten()
        }).flatten()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.get_coordinate_min_max();
        for z in min.2..=max.2 {
            writeln!(f, "Z={}", z)?;
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    let c = match  self.get_cube_at((x,y,z)) {
                        Cube::Active => '#',
                        Cube::Inactive => '.',
                    };
                    f.write_char(c)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn parse_starting_state(s: &str) -> Result<Grid, CubeError> {
    let z = 0;
    let coordinates: Result<HashMap<_, _>, _> = s
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| match c {
                '.' => Ok(((x as isize, y as isize, z), Cube::Inactive)),
                '#' => Ok(((x as isize, y as isize, z), Cube::Active)),
                _ => Err(CubeError::InitialStateError),
            })
        })
        .flatten()
        .collect();

    Ok(Grid {
        coordinates: coordinates?,
    })
}

#[derive(Debug, Clone, Error)]
enum CubeError {
    #[error("could not parse initial state")]
    InitialStateError,
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = ".#.\n..#\n###";
    #[test]
    fn test_parse_grid() {
        let grid = parse_starting_state(EXAMPLE).unwrap();

        let z = 0;
        for (x, y) in &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)] {
            assert_eq!(*grid.coordinates.get(&(*x, *y, z)).unwrap(), Cube::Active);
        }
    }

    // Each cube only ever considers its neighbors: any of the 26 other cubes where any of their
    // coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors
    // include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
    #[test]
    fn test_get_neighbours() {
        let grid = parse_starting_state(EXAMPLE).unwrap();

        let neighbours= grid.get_neighbours((1,1,0)).filter(|c|*c == Cube::Active).count();
        assert_eq!(5, neighbours);

        let neighbours= grid.get_neighbours((1,2,0)).filter(|c|*c == Cube::Active).count();
        assert_eq!(4, neighbours);
    }

    #[test]
    fn test_example_part1() {
        let mut grid = parse_starting_state(EXAMPLE).unwrap();

        // println!("{}", grid);
        grid.step();
        // println!("Cycle1: \n{}", grid);
        grid.step();
        // println!("Cycle2: \n{}", grid);
        grid.step();
        grid.step();
        grid.step();
        grid.step();


        let count = grid.coordinates.values().filter(|c|**c == Cube::Active).count();
        assert_eq!(112, count);
    }
}
