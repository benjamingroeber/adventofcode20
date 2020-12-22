use helpers::read_file;
use itertools::{Itertools, MinMaxResult};
use std::collections::VecDeque;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day22.txt")?;
    let mut game = parse_game(&input)?;

    println!("{:?}", game);
    let winner = game.play();
    // Part 1
    println!("{} score: {}", winner.name, winner.get_score());
    Ok(())
}

type Card = u8;
type Deck = VecDeque<Card>;

#[derive(Copy, Clone, Debug)]
enum State {
    Continue,
    Win(usize),
}

#[derive(Debug, Clone)]
struct Player<'a> {
    name: &'a str,
    deck: Deck,
}

impl<'a> Player<'a> {
    // you can calculate the winning player's score. The bottom card in their deck is worth the
    // value of the card multiplied by 1, the second-from-the-bottom card is worth the value of the
    // card multiplied by 2, and so on. With 10 cards, the top card is worth the value on the card
    // multiplied by 10.
    fn get_score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * *c as usize)
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Game<'a> {
    round: usize,
    players: Vec<Player<'a>>,
}

impl<'a> Game<'a> {
    fn play(&mut self) -> &Player {
        loop {
            let result = self.play_round();
            match result {
                State::Continue => println!("Round: {}", self.round),
                State::Win(w) => return &self.players[w],
            }
        }
    }

    fn play_round(&mut self) -> State {
        self.round += 1;
        let mut next_cards: Vec<_> = self
            .players
            .iter_mut()
            .enumerate()
            .map(|(i, p)| (i, p.deck.pop_front()))
            .collect();

        next_cards.sort_by_key(|c| c.1);

        println!("Cards: {:?}", next_cards);
        if let Some((winner, _)) = next_cards.last() {
            // must be put in deck biggest to smallest
            for (_, card) in next_cards.iter().rev() {
                if let Some(c) = card {
                    println!("{} wins {}", self.players[*winner].name, c);
                    self.players[*winner].deck.push_back(*c)
                }
            }
        }

        // determine state
        // if all but one players have an empty deck, he wins
        if self.players.iter().filter(|p| p.deck.is_empty()).count() < self.players.len() - 1 {
            State::Continue
        } else {
            let (idx, _) = self
                .players
                .iter()
                .enumerate()
                .find(|p| !p.1.deck.is_empty())
                .unwrap();
            State::Win(idx)
        }
    }
    fn with_players(players: Vec<Player<'a>>) -> Result<Game<'a>, CrabCombatError> {
        // sanity check
        match players.iter().map(|p| p.deck.iter()).flatten().minmax() {
            MinMaxResult::MinMax(min, max) => {
                for i in *min..=*max {
                    if players
                        .iter()
                        .map(|p| p.deck.iter())
                        .flatten()
                        .filter(|c| **c == i)
                        .count()
                        != 1
                    {
                        // check that each card in range is contained exactly once
                        // TODO more concise error handling
                        return Err(CrabCombatError::IncompleteDeck);
                    }
                }
            }
            // require at least two cards
            _ => return Err(CrabCombatError::IncompleteDeck),
        }
        Ok(Game { round: 0, players })
    }
}

fn parse_game(input: &str) -> Result<Game, CrabCombatError> {
    let players: Result<Vec<_>, _> = input
        .split("\n\n")
        .map(|section| parse_player(section))
        .collect();
    Game::with_players(players?)
}

fn parse_player(input: &str) -> Result<Player, CrabCombatError> {
    let mut lines = input.lines();
    let name = lines
        .next()
        .ok_or(CrabCombatError::MissingPlayerIdentifier)?
        .trim_end_matches(':');
    let deck: Result<Deck, _> = lines.map(Card::from_str).collect();

    Ok(Player { name, deck: deck? })
}

#[derive(Clone, Debug, Error)]
enum CrabCombatError {
    #[error("could not parse card")]
    UnknownCard(#[from] ParseIntError),
    #[error("missing player identifier")]
    MissingPlayerIdentifier,
    #[error("no complete card deck in players cards")]
    IncompleteDeck,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut game = parse_game(EXAMPLE).unwrap();

        let winner = game.play();

        assert_eq!(winner.name, "Player 2");
        // In this example, the winning player's score is:
        //    3 * 10
        // +  2 *  9
        // + 10 *  8
        // +  6 *  7
        // +  8 *  6
        // +  5 *  5
        // +  9 *  4
        // +  4 *  3
        // +  7 *  2
        // +  1 *  1
        // = 306

        assert_eq!(306, winner.get_score());
    }

    static EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
}
