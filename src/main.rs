use std::io;
fn main(){
    let str="I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut input=String::new();
    let mut count=0;
    loop{
        println!("{str}");
       io::stdin().read_line(&mut input).expect("saisi incorect");
       count+=1;
       if input.trim()==String::from("The letter e"){
        println!("Number of trials: {count}");
        break;
       }
    }
}
