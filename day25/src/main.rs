use helpers::read_file;
use std::error::Error;
use thiserror::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>>{
    let input = read_file("./assets/days/day25.txt")?;
    let (pk1, pk2) = parse_public_keys(&input)?;
    let subject_number = 7;

    // Part 1
    // What encryption key is the handshake trying to establish?

    // Given 7 as subject number, find the loop size to generate the encryption key
    let ls = find_loop_size(subject_number, pk1);
    // Generate the encryption key by applying loop size times the transformation to the
    // other party's public key
    let encryption_key = find_encryption_key(ls, pk2);

    println!("Encryption key: {}", encryption_key);

    Ok(())
}

type Unit = u64;

// The handshake used by the card and the door involves an operation that transforms a subject
// number.
fn find_loop_size(subject_number: Unit, public_key: Unit) -> Unit {
    // To transform a subject number, start with the value 1.
    let mut value = 1;
    // Then, a number of times called the loop size, perform the transformation steps:
    let mut loop_size = 0;
    while value != public_key {
        loop_size += 1;
        value = transform(value, subject_number);
    }
    loop_size
}

// you can use either device's loop size with the other device's public key to calculate the
// encryption key
fn find_encryption_key(loop_size: Unit, subject_number: Unit) -> Unit {
    let mut value = 1;
    for _ in 0..loop_size {
        value = transform(value, subject_number)
    }
    value
}

fn transform(mut value: Unit, subject_number: Unit) -> Unit {
    //     Set the value to itself multiplied by the subject number.
    value *= subject_number;
    //     Set the value to the remainder after dividing the value by 20201227.
    value % 20201227
}

fn parse_public_keys(s: &str) -> Result<(Unit, Unit), ComboBreakerError> {
    let mut lines = s.lines();
    if let Some(p1) = lines.next() {
        if let Some(p2) = lines.next() {
            return Ok((p1.parse()?,p2.parse()?))
        }
    }
    Err(ComboBreakerError::PublicKeyNotFound)
}

#[derive(Clone, Debug, Error)]
enum ComboBreakerError {
    #[error("could not get public keys")]
    PublicKeyNotFound,
    #[error("could not parse public key")]
    ParsePublicKey(#[from] ParseIntError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        // CARD
        // For example, suppose you know that the card's public key is 5764801.
        let card_pub_key = 5764801;
        let initial_subject_number = 7;
        // With a little trial and error, you can work out that the card's loop size must be 8
        let expected_card_loop_size = 8;

        // DOOR
        // suppose you know that the door's public key is 17807724
        let door_pub_key = 17807724;
        // you can determine that the door's loop size is 11, because transforming the initial
        // subject number of 7 with a loop size of 11 produces 17807724
        let expected_door_loop_size = 11;

        let card_loop_size = find_loop_size(initial_subject_number, card_pub_key);
        let door_loop_size = find_loop_size(initial_subject_number, door_pub_key);

        // ENCRYPTION KEY
        let expected_ec = 14897079;
        let card_ec = find_encryption_key(card_loop_size, door_pub_key);
        let door_ec = find_encryption_key(door_loop_size, card_pub_key);

        assert_eq!(expected_door_loop_size, door_loop_size);
        assert_eq!(expected_card_loop_size, card_loop_size);
        // both ways should lead to the same result
        assert_eq!(card_ec, door_ec);
        assert_eq!(expected_ec, card_ec);
    }
}