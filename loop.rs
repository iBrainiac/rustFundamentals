fn main(){
    let mut counter = 0;
    loop{
        counter += 1;
        println!("hello world");

        if counter == 5 {
            break;
        }
    }
}