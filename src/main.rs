use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap};
use tokio;

#[derive(Debug, Clone)]
struct WeatherStations {
    stations: BTreeMap<String, Vec<f32>>
}

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
#[tokio::main]
async fn main() -> io::Result<()> {
    let stationMap:BTreeMap<String, Vec<f32>> = BTreeMap::new();
    let mut result = WeatherStations {
        stations: stationMap
    };
    let path = Path::new("weather_stations.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    

    for line in reader.lines() {
        let line = line?;
        let split_line = line.split(";").collect::<Vec<_>>();
        let k = split_line[0].to_string();
        let v = split_line[1];
        match result.stations.contains_key(&k) {
            true => {
                let u = result.stations.get_mut(&k).unwrap();
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
                        result.stations.insert(k, vec![num]);
                    },
                    Err(_) => {
                        println!("Could not parse '{}'", v);
                    }
                }
            }
        }
    }

    let mut current_letter = 'A';
    while current_letter <= 'Z' {
        let next_letter = ((current_letter as u8) + 1) as char;
        let mut chunk: BTreeMap<String, Vec<f32>> = result.stations.range((current_letter as char).to_string()..
            next_letter.to_string()).map(|(k, v)| (k.clone(), v.clone())).collect();
        let mut _res = result.clone();
        tokio::spawn( async move  {
            
            for (station_name, value) in chunk.iter_mut() {
                let (min, max, median) = calculate_min_max_median(value);
                //println!("Station: {}, Min: {:.2}, Max: {:.2}, Median: {:.2}", station_name, min, max, median);
                _res.stations.insert(station_name.to_string(), vec![min, max, median]);
                // Construct a result (e.g., a struct or a tuple) and append it to global state
                // Append to your global state here
            }
            
        });
         
        current_letter = next_letter;
    }
     println!("{:?}", result);

    Ok(())
}
