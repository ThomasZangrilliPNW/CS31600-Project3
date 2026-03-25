#[derive(Debug)]
enum DaysOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
fn main () {
    let today = DaysOfTheWeek::Monday;
    
    println!("Today is {:?}", today); 
}