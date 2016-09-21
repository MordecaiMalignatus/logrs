use std::io;
use std::io::Write;
use std::fs::OpenOptions;

extern crate chrono;
use self::chrono::*;

pub fn log(entry: String) -> Result<(), io::Error> {
    // steps needed
    // format log entry
    // open file
    // append formatted entry to file
    // clean everything up.

    let now = Local::now();
    let formatted_log = format_log_entry(entry, now);
    let filepath = get_file_name(now.date());

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)
        .unwrap();

    match file.write(formatted_log.as_bytes()) {
        Ok(_) => Ok(()),
        Err(x) => Err(x),
    }
}

fn format_log_entry(entry: String, time: DateTime<Local>) -> String {
    let mut lines = Vec::new();
    let mut copy = entry.clone();
    let timestamp = time.format("%H:%M").to_string();

    // TODO factor this out into .logrs
    let linelength = 80;

    while copy.len() > 0 {
        if copy.len() > linelength {
            let line: String = copy.drain(..linelength).collect();
            lines.push(line);
        } else {
            lines.push(copy.clone());
            copy = String::new();
        }
    }

    let mut res = String::new();
    res.push_str(&timestamp);
    res.push_str(" - \n");
    for line in lines { res.push_str(&line); res.push_str("\n");}
    res.push_str("\n");

    res
}

fn get_file_name(time_of_entry: Date<Local>) -> String {
    time_of_entry.format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod test{
    use super::chrono::*;

    #[test]
    fn file_name_correctness_test() {
        let time = Local.ymd(2000, 1, 1);
        assert_eq!(super::get_file_name(time), "2000-01-01");

        let time_second = Local.ymd(2011, 11, 2);
        assert_eq!(super::get_file_name(time_second), "2011-11-02");
    }
}