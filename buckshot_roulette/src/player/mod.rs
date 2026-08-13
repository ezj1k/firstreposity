use rand::prelude::*;
use rand::seq::IteratorRandom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum ItemType {
    Beer, Loupe, Cigarettes, Saw, Handcuffs, Phone, Pills, Invertor, Adrenalyne,
}

#[derive(Default, Debug)]
pub struct Inventory {
    pub beer: u8,
    pub loupe: u8,
    pub cigarettes: u8,
    pub saw: u8,
    pub handcuffs: u8,
    pub phone: u8,
    pub pills: u8,
    pub invertor: u8,
    pub adrenalyne: u8,
    pub nr_items: u8,
}

pub struct Player {
    pub lives: u8,
    pub items: Inventory,
    pub nr_turns: u8,
}

impl Player {
    pub fn create(nr_lives: u8) -> Self {
        Self {
            lives: nr_lives,
            items: Inventory {..Default::default()},
            nr_turns: 0,
        }
    }

    pub fn add_live(&mut self) {
        if self.lives >= 6 {
            println!("You already have max lives!");
        } else {
            println!("Add 1 life!");
            self.lives += 1;
        }
    }//cigarettes

    pub fn pills(&mut self) {
        let mut rng = rand::rng();
        let probability50 = rng.random::<bool>();

        if probability50 {
            if self.lives >= 6 {
                println!("You already have max lives!");
            }
            else if self.lives == 5 {
                println!("Add 1 life!");
                self.lives += 1;
            }
            else {
                println!("Add 2 lives!");
                self.lives += 2;
            }
        } else {
            self.lives -= 1;
        }
    }//pills

    pub fn create_items(&mut self, round: u8, who: String) {
        let mut rng = rand::rng();
        
        if round < 3 {
            let mut i = 2;
            while i > 0 && self.items.nr_items < 8 {
                let type_item = ItemType::iter().choose(&mut rng).unwrap();
                match type_item {
                    ItemType::Beer => {
                        if self.items.beer < 5 {
                            self.items.beer += 1;
                            println!("{who} got beer");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Loupe => {
                        if self.items.loupe < 5 {
                            self.items.loupe += 1;
                            println!("{who} got loupe");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Cigarettes => {
                        if self.items.cigarettes < 5 {
                            self.items.cigarettes += 1;
                            println!("{who} got cigarettes");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Saw => {
                        if self.items.saw < 5 {
                            self.items.saw += 1;
                            println!("{who} got saw");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Handcuffs => {
                        if self.items.handcuffs < 5 {
                            self.items.handcuffs += 1;
                            println!("{who} got handcuffs");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Phone => {
                        if self.items.phone < 5 {
                            self.items.phone += 1;
                            println!("{who} got phone");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Pills => {
                        if self.items.pills < 5 {
                            self.items.pills += 1;
                            println!("{who} got pills");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Invertor => {
                        if self.items.invertor < 5 {
                            self.items.invertor += 1;
                            println!("{who} got invertor");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Adrenalyne => {
                        if self.items.adrenalyne < 5 {
                            self.items.adrenalyne += 1;
                            println!("{who} got adrenalyne");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                }

                self.items.nr_items += 1;
                if self.items.nr_items == 8 {
                    println!("You already have 8 items");
                    break;
                }
            }
        } else {
            let mut i = 4;
            while i > 0 && self.items.nr_items < 8 {
                let type_item = ItemType::iter().choose(&mut rng).unwrap();
                match type_item {
                    ItemType::Beer => {
                        if self.items.beer < 5 {
                            self.items.beer += 1;
                            println!("{who} got beer");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Loupe => {
                        if self.items.loupe < 5 {
                            self.items.loupe += 1;
                            println!("{who} got loupe");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Cigarettes => {
                        if self.items.cigarettes < 5 {
                            self.items.cigarettes += 1;
                            println!("{who} got cigarettes");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Saw => {
                        if self.items.saw < 5 {
                            self.items.saw += 1;
                            println!("{who} got saw");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Handcuffs => {
                        if self.items.handcuffs < 5 {
                            self.items.handcuffs += 1;
                            println!("{who} got handcuffs");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Phone => {
                        if self.items.phone < 5 {
                            self.items.phone += 1;
                            println!("{who} got phone");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Pills => {
                        if self.items.pills < 5 {
                            self.items.pills += 1;
                            println!("{who} got pills");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Invertor => {
                        if self.items.invertor < 5 {
                            self.items.invertor += 1;
                            println!("{who} got invertor");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                    ItemType::Adrenalyne => {
                        if self.items.adrenalyne < 5 {
                            self.items.adrenalyne += 1;
                            println!("{who} got adrenalyne");
                            i -= 1;
                        } else {
                            continue;
                        }
                    },
                }

                self.items.nr_items += 1;
                if self.items.nr_items == 8 {
                    println!("You already have 8 items");
                    break;
                }
            }
        }
    }
}
