use rand::seq::SliceRandom; // 0.7.2

fn main() {
    let entry_map = vec!["Customs", "Reserve", "Lighthouse", "Woods", "Shoreline", "Interchange", "Streets", "Labs", "Factory"]; 
    let entry_map: Vec<_> = entry_map
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
    println!("{:?}", entry_map);
}