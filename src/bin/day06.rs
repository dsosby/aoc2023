// #![feature(float_next_up_down)]

fn minimum_press(time: f64, distance: f64) -> (f64, f64) {
    let quad = (time.powi(2) - (4.0 * distance)).sqrt();
    // let lower = ((time - quad) / 2.0).next_up().ceil();
    // let upper = ((time + quad) / 2.0).next_down().floor();
    let lower = (((time - quad) / 2.0) + 0.000001).ceil();
    let upper = (((time + quad) / 2.0) - 0.000001).floor();
    
    (lower, upper)
}

fn main() {
  // Sample
    // let games = vec![(7, 9), (15, 40), (30, 200)];

    // Time:        45     98     83     73
    // Distance:   295   1734   1278   1210
    // let games : Vec<(f64, f64)> = vec![(45.0, 295.0), (98.0, 1734.0), (83.0, 1278.0), (73.0, 1210.0)];

    // Part 2
    let games = vec![(45988373.0, 295173412781210.0)];
    
    let result = games.iter()
        .map(|(time, dist)| minimum_press(*time, *dist))
        .map(|(lower, upper)| (upper - lower) + 1.0)
        .fold(1.0, |product, n| n * product);
        
    println!("{}", result);
}