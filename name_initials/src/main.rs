fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

pub fn initials(names: Vec<&str>) -> Vec<String> {
	let mut val=Vec::new();
	
	for name in names {
		let mut string=String::new();
		let inits:Vec<&str>=name.split_whitespace().collect();
	    for i in 0..inits.len(){
			string.push_str(&replace(inits[i]));
			if i!=inits.len()-1{
				string.push_str(" ")
			}

		}
		val.push(string)
	}
	val
}
pub fn replace(s:&str)->String{
	let mut string=String::new();
    let chars:Vec<char>=s.chars().collect();
	for char in chars{
		string.push(char);
		string.push('.');
		break;
	}
    string

}