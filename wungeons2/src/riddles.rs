const RIDDLES_CSV: &str = include_str!("riddles.csv");

pub fn get_riddles_list() -> Vec<(String, String)> {
    RIDDLES_CSV
        .split_terminator("\n")
        .map(|m| {
            let (mut riddle, mut answer) = m.rsplit_once(',').unwrap();

            let binding = riddle.replace("\"", "");
            riddle = &binding;

            (riddle.to_string(), answer.to_string())
        })
        .collect::<Vec<(String, String)>>()
}