mod formant;
use crate::formant::Formant;

fn main() {
    let mut a = Formant{name: String::from("one"), value: 1000};
    let mut b = Formant{name: String::from("two"), value: 5000};
    println!("formant {} - value {}", a.name, a.value);
    println!("formant {} - value {}", b.name, b.value);
    a.value = 2200;
    b.value = 6000;
    println!("formant {} - value {}", a.name, a.value);
    println!("formant {} - value {}", b.name, b.value);
}
