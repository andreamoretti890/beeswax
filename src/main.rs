use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
};

fn main() {
    let file = File::open("measurements.txt").expect("Unable to read the file");
    let memmap = unsafe { memmap2::Mmap::map(&file) }.unwrap();
    let mut map = HashMap::<Vec<u8>, (f64, f64, usize, f64)>::with_capacity(10_000);

    for line in memmap.split(|l| *l == b'\n') {
        if line.is_empty() {
            break;
        }

        let mut fields = line.split(|l| *l == b';');
        let station = fields.next().unwrap();
        let temperature = fields.next().unwrap();
        let temperature: f64 = str::from_utf8(temperature).unwrap().parse().unwrap();

        if let Some(stats) = map.get_mut(station) {
            stats.0 = stats.0.min(temperature);
            stats.1 += temperature;
            stats.2 += 1;
            stats.3 = stats.3.max(temperature);
        } else {
            map.insert(station.to_vec(), (temperature, temperature, 0, temperature));
        }
    }

    print!("{{");
    let stats = BTreeMap::from_iter(map);
    let mut stats = stats.into_iter().peekable();
    while let Some((station, (min, sum, count, max))) = stats.next() {
        print!(
            "{}={min:.1}/{:.1}/{max:.1}",
            str::from_utf8(&station).unwrap(),
            sum / (count as f64)
        );
        if stats.peek().is_some() {
            print!(", ");
        }
    }
    print!("}}");
}
