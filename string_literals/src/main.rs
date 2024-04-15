pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    let chars: Vec<char> = v.chars().collect();
    for i in 0..v.len() {
        if (chars[i] as u32) > 127 {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let mut count = 0;
    let charspat: Vec<char> = pat.chars().collect();
    let charsv: Vec<char> = v.chars().collect();
    if pat.len() > v.len() {
        return false;
    }
    for i in 0..v.len() {
        for j in 0..pat.len() {
            println!("{} {} ", charsv[i + j], charspat[j]);
            if charsv[i + j] == charspat[j] {
                count += 1;
            } else {
                break;
            }
        }
        if count == pat.len() {
            return true;
        }
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let valu = v.len() / index;
    (&v[0..valu], &v[valu..])
}

pub fn find(v: &str, pat: char) -> usize {
    let mut count = 0;
    let charsv: Vec<char> = v.chars().collect();
  
    for i in 0..v.len() {
        if charsv[i] == pat {
            return i;
        }
    }
 return v.len() ;
}

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 't'));
}
