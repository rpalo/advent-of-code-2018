use std::collections::HashMap;

/// Day 4: Repose Record
/// 
/// Track the sleeping times of various security guards

// Part 1: Find the guard who slept most and their most-slept minute

/// The security team (made up of a bunch of Guards) that we're monitoring
struct SecurityTeam {
    guards: HashMap<usize, Guard>,
}

impl SecurityTeam {
    pub fn new() -> Self {
        Self { guards: HashMap::new() }
    }

    /// Loads in a log-file-like schedule from text
    /// 
    /// Entries have the form '[YYYY-MM-DD HH:MM] message'
    /// Possible messages are 'Guard #<id> begins shift'
    ///                       'falls asleep'
    ///                       'wakes up'
    /// Assumes that the logs are in chronological order (WHICH IS SANE)
    pub fn load_schedule(&mut self, text: &str) {
        let mut current_guard = 0;
        let mut current_state = "";
        let mut sleep_start = 0;

        for line in text.lines() {
            // Yeah I'm hard-coding the minutes location, DON'T JUDGE ME
            let minute: usize = line[15..17].parse().expect("Minutes weren't a number");
            let action = &line[19..];
            if action.contains("Guard") {
                // New guard starting shift.
                let id = action.trim_matches(|c: char| !c.is_numeric()).parse().expect("No guard ID");
                self.guards.entry(id).or_insert(Guard::new(id));
                current_guard = id;
                current_state = "awake";
            } else if action.contains("falls asleep") {
                // Ignores double-falls asleep
                if current_state == "awake" {
                    current_state = "asleep";
                    sleep_start = minute;
                }
            } else if action.contains("wakes up") {
                // Ignores double-wakes
                if current_state == "asleep" {
                    self.guards.get_mut(&current_guard).unwrap().track_sleep(sleep_start, minute);
                    current_state = "awake";
                }
            }
        }
    }

    /// Returns the guard with the overall most minutes asleep
    pub fn sleepiest_guard(&self) -> &Guard {
        self.guards.values()
            .max_by_key(|guard| guard.total_minutes_asleep())
            .expect("No guards on team")
    }

    /// Returns the guard that fell asleep the most on the same minute
    pub fn most_consistent_sleeper(&self) -> &Guard {
        self.guards.values()
            .max_by_key(|guard| guard.sleep_on(guard.sleepiest_minute()))
            .expect("No guards on team")
    }
}

/// A security guard.  He keeps track of his own sleep times (what a great person)!
struct Guard {
    id: usize,
    sleep_minutes: HashMap<usize, usize>,
}

impl Guard {
    pub fn new(id: usize) -> Self {
        Self { id, sleep_minutes: HashMap::new() }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the minute in which this guard most commonly slept
    /// 
    /// Accounts for the possibility that this guard doesn't suck at their
    /// job and stays awake the whole time.
    pub fn sleepiest_minute(&self) -> usize {
        *self.sleep_minutes.iter()
            .max_by_key(|(_id, minutes)| **minutes)
            .unwrap_or((&0, &0))
            .0
    }

    /// Sums up this guards total sleeping time
    pub fn total_minutes_asleep(&self) -> usize {
        self.sleep_minutes.values().sum()
    }

    /// Logs in a length of time where the guard was asleep
    pub fn track_sleep(&mut self, asleep: usize, awake: usize) {
        for minute in asleep..awake {
            *self.sleep_minutes.entry(minute).or_insert(0) += 1
        }
    }

    /// Returns the amount of times this guard slept on a given minute
    pub fn sleep_on(&self, minute: usize) -> usize {
        *self.sleep_minutes.get(&minute).unwrap_or(&0)
    }
}

/// Part 1 asks for the ID of the guard who slept the most multiplied by
/// the amount of minutes they slept
pub fn part1(text: &str) -> usize {
    let mut guards = SecurityTeam::new();
    guards.load_schedule(text);
    let sleepy = guards.sleepiest_guard();
    sleepy.id() * sleepy.sleepiest_minute()
}

// Part 2

/// Part two asks for the ID of the guard who slept the most on a single
/// minute multiplied by that minute
pub fn part2(text: &str) -> usize {
    let mut guards = SecurityTeam::new();
    guards.load_schedule(text);
    let consistent_guard = guards.most_consistent_sleeper();
    consistent_guard.id() * consistent_guard.sleepiest_minute()
}


#[cfg(test)]
mod tests {
    use super::*;

        #[test]
    fn test_sleepy_id() {
        let schedule = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        let mut squad = SecurityTeam::new();
        squad.load_schedule(schedule);
        assert_eq!(10, squad.sleepiest_guard().id());
    }

    #[test]
    fn test_part_one() {
        let schedule = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(240, part1(schedule));
    }

        #[test]
    fn test_part_two() {
        let schedule = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(4455, part2(schedule));
    }

    #[test]
    fn test_sleepiest_minute() {
        let schedule = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        let mut squad = SecurityTeam::new();
        squad.load_schedule(schedule);
        assert_eq!(45, squad.guards.get(&99).unwrap().sleepiest_minute());
    }
}