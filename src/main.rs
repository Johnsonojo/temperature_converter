fn main() {
    let temp_value: f64 = -40.0;
    let temp_type: &str = "celsius";

    convert_temperature(temp_value, temp_type);
}


fn convert_temperature(temp_value:f64, temp_type: &str){
    let fahrenheit_value: f64;
    let kelvin_value: f64;
    let celsius_value: f64;

    if temp_type == "celsius" {
        fahrenheit_value = (temp_value * 9.0) / 5.0 + 32.0;
        kelvin_value = temp_value + 273.15;
        println!("{}°C = {}°F", temp_value,fahrenheit_value);
        println!("{}°C = {}K", temp_value,kelvin_value);
    }else if temp_type == "fahrenheit" {
        celsius_value = (temp_value - 32.0) * 5.0 / 9.0;
        kelvin_value = (temp_value + 32.0) * 5.0 / 9.0 + 273.15;
        println!("{}°F = {}°C", temp_value,celsius_value);
        println!("{}°F = {}K", temp_value,kelvin_value);
    } else if temp_type == "kelvin" {
        celsius_value = temp_value - 273.15;
        fahrenheit_value = (temp_value - 273.15) * 9.0 / 5.0 + 32.0;
        println!("{}K = {}°C", temp_value,celsius_value);
        println!("{}K = {}°F", temp_value,fahrenheit_value);
    } else {
        println!("Please enter a valid temperature type");
    }
}