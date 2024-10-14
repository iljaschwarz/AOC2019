pub fn read_input(day: u32) -> String {
    let filename = format!("input/input{:02}.txt", day);
    std::fs::read_to_string(filename).unwrap()
}
