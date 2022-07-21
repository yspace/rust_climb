use std::collections::BTreeMap;

pub fn main() {
    let mut voc = BTreeMap::new();

    voc.insert(3_697_915, "Amsterdam");
    voc.insert(1_300_405, "Middelburg");
    voc.insert(540_000, "Enkhuizen");
    voc.insert(469_400, "Delft");
    voc.insert(266_868, "Hoorn");
    voc.insert(173_000, "Rotterdam");

    for (guilders, kamer) in &voc {
       
        println!("{} invested {}", kamer, guilders);
    }

    println!("smaller chambers: ") ;
 // BTreeMap lets you select a portion of the keys that are iterated through with the range syntax.
    for (_guilder, karma) in voc.range(0..=500_000) {
        println!("{}", karma);
    }
    println!("");
}
 
