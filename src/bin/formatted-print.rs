fn main () {
    //Tulostaa '31 days' (Aalto sulkeet täydenetään pilkun jälkeisellä muuttujalla)
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    //Tulostaa Alice tämä on Bob. Bob tämä on Alice
    // Printissä ensimmäinen argumentti tulostetaan. Seuraavat argumentit ovat Array \n
    //Lista josta voidaan jotka voidaan indexejä hyödyntämällä sijoitaa aaltosulkeisiin.

//Erinomainen tapa näyttää kuinka muuttujia sa käytettyä merkkijonossa
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

            //Oma testi, toimii hyvin myös makron ulkopuolisille muuttujille
let verb: &'static str = "jumps over";
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox");
        }
    
    