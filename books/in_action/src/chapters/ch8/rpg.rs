use rand ;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf{}

#[derive(Debug)]
struct Elf{}

#[derive(Debug)]
struct Human{}

#[derive(Debug)]
enum Thing{
    Sword ,
    Thrinket ,

}

trait Enchanter: std::fmt::Debug{
    fn completency(&self) -> f64 ;

    fn enchant(&self, thing: &mut Thing){
        let probability_of_success = self.completency();
        let spell_is_successful = rand::thread_rng()
        .gen_bool(probability_of_success);
        println!("{:?} mutters incoherently.", self); 
        if spell_is_successful{
            println!("The {:?} glows brightly.", thing);
            
         }else{
            println!("The {:?} fizzes, \
            then turns into a worthless trinket.", thing);
            *thing = Thing::Thrinket;
         }
    }
}

impl Enchanter for Dwarf{
    fn completency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf{
    fn completency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human{
    fn completency(&self) -> f64 {
        0.8
    }
}

pub fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf{};
    let e = Elf{};
    let h = Human{};

    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it) ;
}