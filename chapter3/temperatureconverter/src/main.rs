fn main() {
    let temp = get_input();
    let new_temp = convert_temp(temp);

    println!("New temperature is {new_temp}");
}

fn get_input() -> Temperature {
    println!("Please enter a temperature in format: TempUnit. Ex. 32F, 18C");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if input.len() < 2 {
        panic!("Input {input} was not in the proper format.");
    }

    let temp: f64 = input[..input.len()-1]
        .trim()
        .parse()
        .expect("Temperature was not in the correct format");
    let unit = &input[input.len()-1..];

    let temp_type = match unit.to_lowercase().as_str() {
        "f" => TempType::Farenheight,
        "c" => TempType::Celcius,
        _ => panic!("Unknown temperature type")
    };

    Temperature {
        temp: temp,
        temp_type: temp_type
    }
}

fn convert_temp(temp: Temperature) -> Temperature {
    let new_temp = match temp.temp_type {
        TempType::Farenheight => farenheight_to_celcius(temp.temp),
        TempType::Celcius => celcius_to_farenheight(temp.temp)
    };

    let new_temp_type = match temp.temp_type {
        TempType::Farenheight => TempType::Celcius,
        TempType::Celcius => TempType::Farenheight
    };

    Temperature {
        temp: new_temp,
        temp_type: new_temp_type
    }
}

fn farenheight_to_celcius(f_temp: f64) -> f64 {
    (5.0 / 9.0) * (f_temp - 32.0)
}

fn celcius_to_farenheight(c_temp: f64) -> f64 {
    (c_temp * (9.0 / 5.0)) + 32.0
}

struct Temperature {
    temp: f64,
    temp_type: TempType
}

enum TempType {
    Farenheight = 0,
    Celcius = 1
}

impl std::fmt::Display for TempType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TempType::Farenheight => write!(f, "°F"),
            TempType::Celcius => write!(f, "°C")
        }
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.temp, self.temp_type)
    }
}
