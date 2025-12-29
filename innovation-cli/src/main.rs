use innovation_core::{load_all_cards, Color};
use std::collections::HashMap;

fn main() {
    let cards = load_all_cards();
    println!("Total cards loaded: {}", cards.len());

    let mut by_age: HashMap<u8, HashMap<Color, usize>> = HashMap::new();

    for card in &cards {
        by_age
            .entry(card.age)
            .or_default()
            .entry(card.color)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut ages: Vec<_> = by_age.keys().cloned().collect();
    ages.sort();

    println!("\nDistribution by Age and Color:");
    for age in ages {
        println!("Age {}:", age);
        let colors = &by_age[&age];
        let mut color_names: Vec<_> = colors.keys().collect();
        // Custom sort for consistent output (Red, Blue, Green, Yellow, Purple if possible, or alphabetical)
        color_names.sort_by_key(|c| format!("{:?}", c)); 

        for color in color_names {
            println!("  {:?}: {}", color, colors[color]);
        }
        let total_age: usize = colors.values().sum();
        println!("  Total: {}", total_age);
    }
}

