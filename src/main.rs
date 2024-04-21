use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{btree_map, BTreeMap};
fn calculate_min_max_median(numbers: &mut Vec<f32>) -> (f32, f32, f32) {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = *sorted_numbers.first().unwrap();
    let max = *sorted_numbers.last().unwrap();

    let mid = sorted_numbers.len() / 2;
    let median = if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) as f32 / 2.0
    } else {
        sorted_numbers[mid] as f32
    };

    (min as f32, max as f32, median)
}
fn main() -> io::Result<()> {
    let path = Path::new("weather_stations.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut stationMap:BTreeMap<String, Vec<f32>> = btree_map::BTreeMap::new();

    for line in reader.lines() {
        let line = line?;
        let split_line = line.split(";").collect::<Vec<_>>();
        let k = split_line[0];
        let v = split_line[1];
        match stationMap.contains_key(k) {
            true => {
                let u = stationMap.get_mut(k).unwrap();
                match v.parse::<f32>() {
                    Ok(num) => {
                        u.push(num);
                    },
                    Err(_) => {
                        println!("Could not parse '{}'", v);
                    }
                }
            }
            false => {
                match v.parse::<f32>() {
                    Ok(num) => {
                        stationMap.insert(k.parse().unwrap(), vec![num]);
                    },
                    Err(_) => {
                        println!("Could not parse '{}'", v);
                    }
                }
            }
            _ => println!("invalid")
        }

    }

    for (_key, value) in stationMap.iter_mut() {
        let res = calculate_min_max_median(value);
        println!("{}, {:?}", _key, res);
    }
    Ok(())
}
