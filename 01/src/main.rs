use std::fs::File;
use std::io::{self, BufReader, BufRead};

mod fuel;
use fuel::total_required_fuel;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let file = File::open("./src/modules.txt")?;
    let reader = BufReader::new(file);

    let mut modules: Vec<i64> = vec!();
    for line in reader.lines() {
        let line = line?;
        let line = line.as_str();

        let module = i64::from_str(line);

        match module {
            Ok(mass) => modules.push(mass),
            _ => panic!("Could not convert {} to i64", line),
        }
    }

    let total_fuel = total_required_fuel(&modules, false);
    println!("Total fuel required: {}", total_fuel);

    let total_fuel_inc_fuel_weight = total_required_fuel(&modules, true);
    println!("Total fuel required including fuel weight: {}", total_fuel_inc_fuel_weight);

    Ok(())
}
