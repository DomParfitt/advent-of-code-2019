pub fn total_required_fuel(modules: &Vec<i64>, include_fuel_weight: bool) -> i64 {
    let mut fuel = 0;

    for &module in modules {
        fuel += if include_fuel_weight {
            required_fuel_acc(module, 0)
        } else {
            required_fuel(module)
        }
    }

    fuel
}

fn required_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn required_fuel_acc(mass: i64, acc: i64) -> i64 {
    let fuel = required_fuel(mass);
    if fuel <= 0 {
        acc
    } else {
        required_fuel_acc(fuel, acc + fuel)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_required_fuel() {
        let mut tests: HashMap<i64, i64> = HashMap::new();
        tests.insert(12, 2);
        tests.insert(14, 2);
        tests.insert(1969, 654);
        tests.insert(100756, 33583);

        for (mass, fuel) in tests {
            assert_eq!(required_fuel(mass), fuel);
        }
    }

    #[test]
    fn test_required_inc_fuel_weight() {
        let mut tests: HashMap<i64, i64> = HashMap::new();
        tests.insert(12, 2);
        tests.insert(14, 2);
        tests.insert(1969, 966);
        tests.insert(100756, 50346);

        for (mass, fuel) in tests {
//            println!("Mass {}", mass);
            assert_eq!(required_fuel_acc(mass, 0), fuel);
        }
    }
}