/// Return the distance between two points in meters
fn distance_between_two_points(a: &[f64; 2], b: &[f64; 2]) -> f64 {
    let a = haversine::Location { latitude: a[0], longitude: a[1] };
    let b = haversine::Location { latitude: b[0], longitude: b[1] };

    haversine::distance(a, b, haversine::Units::Kilometers) * 1000.
}

fn main() {
    let args = std::env::args().skip(1).take(4).map(|arg| arg.trim().trim_matches(',').parse()).collect::<Result<Vec<f64>, _>>().unwrap();
    if args.len() != 4 {
        panic!("expected 4 args, got {}", args.len());
    }

    let points: Vec<[f64; 2]> = args.chunks_exact(2).map(|arg| [arg[0], arg[1]]).collect();

    println!("There is a distance of {} meters between your two points", distance_between_two_points(&points[0], &points[1]));
}
