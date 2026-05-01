fn main() {
    let days = [
        "first", 
        "second", 
        "third", 
        "fourth",
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth",
        "ninth", 
        "tenth", 
        "eleventh", 
        "twelfth",
    ];

    let items = [
        "a partridge in a pear tree",
        "two turtle doves and",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..12 
    {
        println!("[verse {}]", i + 1);

        println!("on the {} day of christmas, my true love sent to me", days[i]);

        for j in 0..=i 
        {
            let index = i - j;

            println!("{}", items[index]);

        }

        println!();
    }
}