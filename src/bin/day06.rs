// #![feature(float_next_up_down)]

fn minimum_press(time: u32, distance: u32) -> (u32, u32) {
    let time: f64 = time.into();
    let distance: f64 = distance.into();

    let quad = (time.powi(2) - (4.0 * distance)).sqrt();
    // let lower = ((time - quad) / 2.0).next_up().ceil() as u32;
    // let upper = ((time + quad) / 2.0).next_down().floor() as u32;
    let lower = (((time - quad) / 2.0) + 0.000001).ceil() as u32;
    let upper = (((time + quad) / 2.0) - 0.000001).floor() as u32;
    
    (lower, upper)
}

fn main() {
  // Sample
    // let games = vec![(7, 9), (15, 40), (30, 200)];

    // Time:        45     98     83     73
    // Distance:   295   1734   1278   1210
    let games = vec![(45, 295), (98, 1734), (83, 1278), (73, 1210)];

    // Part 2
    // let games = vec![(79154030200, 45295981734831278731210)];
    
    let result = games.iter()
        .map(|(time, dist)| minimum_press(*time, *dist))
        .map(|(lower, upper)| (upper - lower) + 1)
        .fold(1, |product, n| n * product);
        
    println!("{}", result);
}