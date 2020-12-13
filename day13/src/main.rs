use helpers::read_file;
use std::error::Error;
use std::num::ParseIntError;
use thiserror::Error;

type Unit = isize;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day13.txt")?;
    let (start_ts, scheduled_busses) = parse_timestamp_and_schedules(&input)?;

    // Part 1
    // What is the ID of the earliest bus you can take to the airport multiplied by the number of
    // minutes you'll need to wait for that bus?
    let (first_ts, bus) = (start_ts..)
        .filter_map(|timestamp| {
            scheduled_busses
                .iter()
                .find(|bus| timestamp % bus.interval == 0)
                .map(|schedule| (timestamp, schedule))
        })
        .next()
        .unwrap();

    println!(
        "First departure {} at Bus {}: Waiting Time * Bus Interval = {}",
        first_ts,
        bus.interval,
        (first_ts - start_ts) * bus.interval
    );

    // Part 2
    // What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching
    // their positions in the list?
    let mut step = 1;
    let mut timestamp = 0;
    // we sieve through the timestamps by finding a solution for each bus
    // when each bus is solved, the condition is reached
    // combining the solutions is possible by increasing the step size, stepping in multiples of the
    // bus intervals, once a solution is found
    for bus in &scheduled_busses {
        // step in the future until conditions are satisfied also for the current bus
        while (timestamp + bus.departure_offset) % bus.interval != 0 {
            timestamp += step;
        }

        step *= bus.interval;
    }

    println!(
        "First time when all busses depart at their correct offsets {}",
        timestamp
    );

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct ScheduledBus {
    departure_offset: Unit,
    interval: Unit,
}

fn parse_timestamp_and_schedules(s: &str) -> Result<(Unit, Vec<ScheduledBus>), ScheduleError> {
    let mut lines = s.lines();

    let timestamp = lines
        .next()
        .map(str::parse::<Unit>)
        .ok_or(ScheduleError::MissingTimestamp)?;

    let schedule = lines.next().ok_or(ScheduleError::MissingSchedules)?;

    let busses: Result<Vec<_>, _> = schedule
        .split(',')
        .enumerate()
        .filter(|s| s.1 != "x")
        // .inspect(|c| println!("{:?}", c))
        .map(|(idx, interval)| {
            interval.parse().map(|interval| ScheduledBus {
                departure_offset: idx as isize,
                interval,
            })
        })
        .collect();

    Ok((timestamp?, busses?))
}

#[derive(Clone, Debug, Error)]
enum ScheduleError {
    #[error("timestamp not found")]
    MissingTimestamp,
    #[error("schedules not found")]
    MissingSchedules,
    #[error("could not parse schedule")]
    Parser(#[from] ParseIntError),
}
