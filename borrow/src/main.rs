pub fn str_len(s: &str ) -> usize {
	
	let chars: Vec<char> = s.chars().collect();
	let mut count :usize= 0;
	
	while !chars.get(count).is_none() {
		count+=1;
	}
	count
}


fn main() {
	let s = "hello";
	let s1 = "camelCase".to_string();

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
}