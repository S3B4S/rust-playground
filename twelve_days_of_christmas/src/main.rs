fn main() {
    let main_text = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four colly birds",
        "Five gold rings",
    ];

    let words = ["first", "second", "third", "fourth", "fifth"];

    for (i, word) in words.iter().enumerate() {
        println!("On the {} day of christmas my true love sent me", word);

        let end = i + 1;
        for number in (0..end).rev() {
            println!("{}", main_text[number]);
        }

        println!("");
    }
}
