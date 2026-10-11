use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("measurements.txt").expect("Unable to read the file");
    let reader = BufReader::with_capacity(10_000_000_000, file);
    let mut map = HashMap::<String, (f64, f64, usize, f64)>::with_capacity(10_000);

    for line in reader.lines() {
        let line = line.unwrap();
        let (station, temperature) = line.split_once(';').unwrap();
        let temperature: f64 = temperature.parse::<f64>().unwrap();
        let stats = map
            .entry(station.to_string())
            .or_insert((f64::MAX, 0.0, 0, f64::MIN));
        stats.0 = stats.0.min(temperature);
        stats.1 += temperature;
        stats.2 += 1;
        stats.3 = stats.3.max(temperature);
    }

    print!("{{");
    let stats = BTreeMap::from_iter(map);
    let mut stats = stats.into_iter().peekable();
    while let Some((station, (min, sum, count, max))) = stats.next() {
        print!("{station}={min:.1}/{:.1}/{max:.1}", sum / (count as f64));
        if stats.peek().is_some() {
            print!(", ");
        }
    }
    print!("}}");
}
