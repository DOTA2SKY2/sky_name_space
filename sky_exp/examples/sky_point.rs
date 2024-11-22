use std::fmt;

// #[derive(Debug)]
struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.longitude)
            .field(&self.longitude)
            .field(&self.latitude)
            .finish()
    }
}

fn main() {
    let position = Position { longitude: 1.987, latitude: 2.983 };
    println!("{position:?}");
    // assert_eq!(format!("{position:?}"), "(1.987, 2.983)");

//     assert_eq!(format!("{position:#?}"), "(
//     1.987,
//     2.983,
// )");
}

