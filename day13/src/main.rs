use helpers::read_file;
use std::error::Error;
use std::num::ParseIntError;
use thiserror::Error;

type Unit = isize;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day13.txt")?;
    let (start_ts, scheduled_busses) = parse_timestamp_and_schedules(&input)?;

    let (first_ts, bus) = (start_ts..)
        .filter_map(|ts| {
            scheduled_busses
                .iter()
                .find(|bus| ts % bus.interval == 0)
                .map(|schedule| (ts, schedule))
        })
        .next()
        .unwrap();

    // Part 1
    // What is the ID of the earliest bus you can take to the airport multiplied by the number of
    // minutes you'll need to wait for that bus?
    println!(
        "First departure {} at Bus {} - Diff * Bus = {}",
        first_ts,
        bus.interval,
        (first_ts - start_ts) * bus.interval
    );
    // println!("{}\n{:?} - {:?}", start_ts, schedules, first);

    // Part 2
    // No working solution yet
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct ScheduledBus {
    idx: Unit,
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
                idx: idx as isize,
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
