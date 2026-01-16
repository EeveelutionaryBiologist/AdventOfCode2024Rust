use regex::Regex;
use std::fs;

fn read_input() -> String {
    let input = fs::read_to_string("scrambled_code.txt")
        .expect("No file found.")
        .replace('\n', "");
    split_by_commands(input)
}

fn split_by_commands(text: String) -> String {
    let chunks: Vec<&str> = text.split("do()").collect();
    chunks
        .into_iter()
        .map(|s| s.split("don't()").next().unwrap_or(""))
        .collect::<String>()
}

fn main() {
    // let example = "'*when(932,461)!:who()(*mul(662,950)mul(878,53){#+{&%}mul(675,225)^how(648,963)&)#how(924,189)[mul(14,114)!mul(528,270)]#from()mul(866,868) what())mul(253,100); )when()@{mul(827,104))% -,'mul(955,284)/";
    let text = read_input();
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    let mut mult_results: Vec<i32> = Vec::new();

    for caps in re.captures_iter(&text) {
        // caps[0] is the whole match -
        // caps[1] is the first capture group -
        let values: Vec<i32> = caps[1]
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        mult_results.push(values[0] * values[1]);
    }
    if mult_results.len() > 0 {
        println!(
            "Sum of mul() statements: {}",
            mult_results.into_iter().sum::<i32>()
        );
    }
}
