pub fn first_subword(mut s: String) -> String {
	let chars:Vec<char>=s.chars().collect();
	let mut string=String::new();
	string.push_str(&s[0..indice(&s)]);
	string
}

pub fn indice(s: &String) -> usize {
	let chars:Vec<char>=s.chars().collect();
	for i in 0..chars.len(){
		if chars[i].is_uppercase() && i!=0 || chars[i]=='_'{
             return i
		} 
	}
	return s.len()
}
fn main() {
	let s1 = String::from("helloWorld");
	let s2 = String::from("snake_case");
	let s3 = String::from("CamelCase");
	let s4 = String::from("just");

	println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
	println!("first_subword({}) = {}", s2.clone(), first_subword(s2));
	println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
	println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
}