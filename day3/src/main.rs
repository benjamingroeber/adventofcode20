use helpers::read_file;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day3.txt");
    let grid = Grid::from_input(&input?)?;

    // Part 1:
    // Starting at the top-left corner of your map and following a slope of right 3 and down 1,
    // how many trees would you encounter?
    println!(
        "Encountered Trees with slope 3,1 => {}",
        grid.count_trees_in_slope(3, 1)
    );

    // Part 2:
    // Determine the number of trees you would encounter if, for each of the following slopes,
    // you start at the top-left corner and traverse the map all the way to the bottom:
    //
    //     Right 1, down 1.
    //     Right 3, down 1. (This is the slope you already checked.)
    //     Right 5, down 1.
    //     Right 7, down 1.
    //     Right 1, down 2.
    //
    // What do you get if you multiply together the number of trees encountered on each of the listed slopes?
    let product: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| grid.count_trees_in_slope(*x, *y))
        .product();
    println!(
        "Product of encountered trees for slopes 1,1 3,1 5,1 7,1 1,2 => {}",
        product
    );
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Square {
    Empty,
    Tree,
}

impl Square {
    fn is_tree(&self) -> bool {
        *self == Square::Tree
    }
}

// # stands for Tree
// . stands for Empty
impl TryFrom<char> for Square {
    type Error = GridError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Square::Tree),
            '.' => Ok(Square::Empty),
            _ => Err(GridError::ParseSquare(value)),
        }
    }
}

// due to something you read about once involving arboreal genetics and biome stability,
// the same pattern repeats to the right many times
#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    nodes: Vec<Vec<Square>>,
}

impl Grid {
    // A map (your puzzle input) of the open squares (.) and trees (#)
    fn from_input(input: &str) -> Result<Self, GridError> {
        let capacity = input.lines().count();

        let mut nodes = Vec::with_capacity(capacity);
        for line in input.lines() {
            let squares: Result<Vec<Square>, _> = line.chars().map(|c| c.try_into()).collect();
            nodes.push(squares?)
        }

        Ok(Grid { nodes })
    }

    // the grid extends infinitely to the right
    fn get_coordinates(&self, x: usize, y: usize) -> Option<&Square> {
        self.nodes.get(y).and_then(|row| row.get(x % row.len()))
    }

    // this iterator yields infinite amount of Squares if y = 0
    // starting point is always square at 0,0
    fn slope(&self, x: usize, y: usize) -> SlopeIterator {
        SlopeIterator {
            grid: self,
            x,
            y,
            iteration: 0,
        }
    }

    fn count_trees_in_slope(&self, x: usize, y: usize) -> usize {
        self.slope(x, y).filter(|c| c.is_tree()).count()
    }
}

struct SlopeIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    iteration: usize,
}

impl<'a> Iterator for SlopeIterator<'a> {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .grid
            .get_coordinates(self.x * self.iteration, self.y * self.iteration)
            .copied();
        self.iteration += 1;
        next
    }
}

#[derive(Error, Debug)]
pub enum GridError {
    #[error("could not parse valid square from `{0}`")]
    ParseSquare(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = ".#.\n#.#";

        let expected_nodes = vec![
            vec![Square::Empty, Square::Tree, Square::Empty],
            vec![Square::Tree, Square::Empty, Square::Tree],
        ];
        let expected = Grid {
            nodes: expected_nodes,
        };
        let parsed = Grid::from_input(input).unwrap();
        assert_eq!(expected, parsed);

        let input = "asd#.f";
        assert!(Grid::from_input(input).is_err())
    }

    #[test]
    fn test_slope() {
        let input = "#....\n#....\n#....\n#....\n#...#";
        let parsed = Grid::from_input(input).unwrap();

        let squares: Vec<_> = parsed.slope(0, 1).collect();
        assert_eq!(vec![Square::Tree; 5], squares);

        let squares: Vec<_> = parsed.slope(1, 1).collect();
        assert_eq!(
            vec![
                Square::Tree,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Tree
            ],
            squares
        );

        let squares: Vec<_> = parsed.slope(2, 2).collect();
        assert_eq!(vec![Square::Tree, Square::Empty, Square::Tree], squares);
    }

    // In this example, traversing the map using this slope would cause you to encounter 7 trees.
    #[test]
    fn test_day1_example() {
        let input = read_file("../assets/days/day3_example.txt").unwrap();
        let grid = Grid::from_input(&input).unwrap();

        let trees_encountered = grid.count_trees_in_slope(3, 1);
        assert_eq!(7, trees_encountered);
    }

    // Determine the number of trees you would encounter if, for each of the following slopes,
    // you start at the top-left corner and traverse the map all the way to the bottom:
    //
    //     Right 1, down 1.
    //     Right 3, down 1. (This is the slope you already checked.)
    //     Right 5, down 1.
    //     Right 7, down 1.
    //     Right 1, down 2.
    #[test]
    fn test_day2_example() {
        let input = read_file("../assets/days/day3_example.txt").unwrap();
        let grid = Grid::from_input(&input).unwrap();

        let trees_encountered: Vec<_> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(x, y)| grid.count_trees_in_slope(*x, *y))
            .collect();

        // In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively;
        assert_eq!(trees_encountered, vec![2, 7, 3, 4, 2]);
        // multiplied together, these produce the answer 336.
        assert_eq!(336_usize, trees_encountered.iter().cloned().product());
    }
}
