pub fn factorial(num: u64) -> u64 {
	let mut f : u64=1;

	if num ==0 || num ==1{
		return 1;
	}
	for i in 1..=num{
		f*= i
	}

	f
}


fn main() {
    println!("The factorial of 0 = {}", factorial(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial(19));
}