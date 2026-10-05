fn main() {
    let text = "free money for my honey, bitcoin";
    let v = vec!["free money", "bitcoin", "urgent"];
    let mut scam_num  = 0;
    for elem in &v{
        if text.contains(elem){
            scam_num += 1;
        }
    }
    let persent_scam = ((scam_num as f64 / v.len() as f64) * 100.0) as i32;
    println!("Scam num: {scam_num}");
    println!("Scam %: {}", persent_scam);

}
