pub fn to_url(s: &str) -> String {

	let chars : Vec<char> = s.chars().collect();
	let mut string =String::new();

	for i in 0..chars.len(){
		if chars[i]==' '{
			string.push('%');
			string.push('2');
			string.push('0');
		}else {
			string.push(chars[i]);
		}
	}
	string
}


fn main() {
	let s = " Hello, world!";
	println!("{} to be use as an url is {}", s, to_url(s));
}