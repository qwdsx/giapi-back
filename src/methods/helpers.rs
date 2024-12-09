
pub fn vec_from_stat_string(string: &String) -> Vec<f32> {
    string.split(" ")
        .map(|i| i.parse::<f32>().unwrap())
        .collect::<Vec<f32>>()
}