macro_rules! five
{
    () => {
        5
    }
}

fn main()
{
    let x = five!();
    let y = five!();
    
    let sum = x + y;

    println!("sum = {}", sum);
}
