#[derive(Debug)]
struct Matrix((i32,i32), (i32,i32));

pub fn transpose(m: Matrix)-> Matrix{

	let a00=m.0.0;
	let a01=m.0.1;
	let a10=m.1.0;
	let a11=m.1.1;

	let matrix= Matrix((a00,a10), (a01,a11));

	matrix
}

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}