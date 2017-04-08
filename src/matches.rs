pub fn match_statement() {
    let country_code = 999; // assume 1..1000

    let country = match country_code {
        44 => "uk",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "unknown", // 1..99 1-998, 1...999 1-999
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);
}
