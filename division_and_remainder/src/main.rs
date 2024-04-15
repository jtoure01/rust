pub fn divide(x: i32, y: i32) -> (i32, i32) {
	let div = x/y;
	let remainder= x%y;

	(div, remainder)
}

fn main() {
    let x = 9;
    let y = 4;
    let (division, remainder) = divide(x, y);
    println!(
        "{}/{}: division = {}, remainder = {}",
        x, y, division, remainder
    );
}