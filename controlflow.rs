pub fn while_loop(name: &mut String)->&str{
    let mut counter = 0;
    while counter < 5 {
        println!("The value of the counter is : {counter}, {name}");
        counter+=1;
    }
    "The while loop control flow has executed completely."
}
pub fn looping()
{
    let mut counter = 2;
    loop {
        if counter < 3 {
            println!("I like food!");
            counter += 1
        }
        else {
            println!("This is the last of the loop");
            break;
        }
    }
}