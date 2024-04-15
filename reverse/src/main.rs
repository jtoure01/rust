
fn main() {
    println!("{}", rev_str("Hello, my name is Roman"));
    println!("{}", rev_str("I have a nice car!"));
    println!("{}", rev_str("How old are You"));
    println!("{}", rev_str("ex: this is an example água"));
}

pub fn rev_str(input: &str)  -> String{
	let mut chars: Vec<char> = input.chars().collect();

    let mut string = String::new();
	let mut count= chars.len() - 1;

	while count !=0 {
		string.push(chars[count]);
		count-=1;
	}
	string.push(chars[count]);

	string.to_string()

}
