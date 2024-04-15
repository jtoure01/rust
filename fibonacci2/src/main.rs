pub fn fibonacci(n: u32) -> u32 {

	if n == 0{
		return 0;
	}

	if n == 1{
		return 1;
	}

	let mut a : u32= 0;
	let mut b : u32= 1;
	let mut c : u32= 0;

	for _i in 0..n{
		a=b;
		b=c;
		c= a+b;
	}

	c
}

fn main() {
    println!("The element in the position {} in fibonacci series is {}",2, fibonacci(2));
    println!("The element in the position {} in fibonacci series is {}",4, fibonacci(4));
    println!("The element in the position {} in fibonacci series is {}",22, fibonacci(22));
    println!("The element in the position {} in fibonacci series is {}", 20, fibonacci(20));
}