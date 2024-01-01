mod generic_struct;

use generic_struct::Point;
fn main() {
    let origin = Point { x: 0, y: 1 };
    let char_origin  = Point { x: 'a', y: 'b' };
    print!("X: {} Y: {}\n", origin.x(), origin.y());
    print!("X: {}, Y: {}\n", char_origin.x, char_origin.y());
}
