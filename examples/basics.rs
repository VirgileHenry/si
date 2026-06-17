fn main() {
    /* Create a new distance value (metres) */
    let distance: si::Quantity<f32, si::unit::std::Metre> = si::Quantity::new(12.0);
    /* Create a new time value (seconds) */
    let time: si::Quantity<f32, si::unit::std::Second> = si::Quantity::new(3.0);

    /* Compute the velocity (metres per second) */
    let velocity = distance / time;

    println!("Travelled {} in {}, going at {}", distance, time, velocity);
}
