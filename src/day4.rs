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
                if current_state == "awake" {
                    current_state = "asleep";
                    sleep_start = minute;
                }
                // Ignores double-falls asleep
            } else if action.contains("wakes up") {
                if current_state == "asleep" {
                    self.guards.get_mut(&current_guard).unwrap().track_sleep(sleep_start, minute);
                    current_state = "awake";
                }
            }
        }
    }

    pub fn sleepiest_guard(&self) -> &Guard {
        self.guards.iter().
            max_by_key(|(_id, guard)| guard.total_minutes_asleep()).expect("No guards on team")
            .1
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

    pub fn sleepiest_minute(&self) -> usize {
        *self.sleep_minutes.iter()
            .max_by_key(|(_id, minutes)| **minutes).expect("Guard didn't sleep")
            .0
    }

    pub fn total_minutes_asleep(&self) -> usize {
        self.sleep_minutes.values().sum()
    }

    pub fn track_sleep(&mut self, asleep: usize, awake: usize) {
        for minute in asleep..awake {
            *self.sleep_minutes.entry(minute).or_insert(0) += 1
        }
    }
}

pub fn part1(text: &str) -> usize {
    let mut guards = SecurityTeam::new();
    guards.load_schedule(text);
    let sleepy = guards.sleepiest_guard();
    sleepy.id() * sleepy.sleepiest_minute()
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
}