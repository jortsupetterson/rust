fn main () {
    // Tulostaa: '31 days'
    println!("{} days", 31);

    // Tulostaa: 'Alice, this is Bob. Bob, this is Alice'
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Tulostaa: 'the quick brown fox jumps over the lazy dog'
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //Testi toimiiko myös ulkopuolisilla muuttujilla: toimii!
    let verb: &'static str = "jumps over";
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox"
    );

    // Täyttö vasemmalta nollilla "00001"
    println!("{number:0>5}", number = 1);

    // Täyttö oikealta nollilla "10000"
    println!("{number:0<5}", number = 1);


    println!("{number:0>width$}", number=1, width=5);

//Tässä vaan testattiin miten debuger toimii jos makrolta puuttuu argumentteja
    print!("My name is {0}, {1} {0}", "Bond", "James");
}


