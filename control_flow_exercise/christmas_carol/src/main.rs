// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree

// On the second day of Christmas, my true love sent to me
// Two turtle doves, and
// A partridge in a pear tree

// On the third day of Christmas, my true love sent to me
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the fourth day of Christmas, my true love sent to me
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the fifth day of Christmas, my true love sent to me
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the sixth day of Christmas, my true love sent to me
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the seventh day of Christmas, my true love sent to me
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the eighth day of Christmas, my true love sent to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the ninth day of Christmas, my true love sent to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the tenth day of Christmas, my true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the eleventh day of Christmas, my true love sent to me
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// On the twelfth day of Christmas, my true love sent to me

fn main() {
    let poem = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let days = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth",
        "Tenth", "Eleventh", "Twelveth",
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {day} day of christmas my tru love gave to me");
        for i in (0..=index).rev() {
            println!("{}", poem[i]);
        }
    }
}
