fn main() {
    println!("Hello, world!");
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("Think Different!");
    }
    else {
        println!("That's also great.");
    }
    if cfg!(target_os = "linux") {
        println!("You go Glen Coco.");
    }
}
