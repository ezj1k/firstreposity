use crate::player::{Player};
use crate::gun::{Gun, Bullet};
use crate::level::Levels;
use crate::utils;

pub struct Game {
    pub player: Player,
    pub dealer: Player,
    pub gun: Gun,
    pub current_lvl: Levels,
    pub round: u8,
}

impl Game {
    pub fn new(level: Levels) -> Self {
        let max_lives; 
        let bullets;
        match level {
            Levels::Level1 => {
                max_lives = 2;
                bullets = Gun::create(0, 2);
            },
            Levels::Level2 => {
                max_lives = 4;
                bullets = Gun::create(3, 5);
            },
            Levels::Level3 => {
                max_lives = 6;
                bullets = Gun::create(5, 7);
            },
        }

        Self {
            player: Player::create(max_lives),
            dealer: Player::create(max_lives),
            gun: Gun {shootgun: bullets},
            current_lvl: level,
            round: 1,
        }
    }

    pub fn start(&mut self) {
        self.player.nr_turns += 1;
        println!("--- Game starts ---");
        println!("Total bullets: {}", self.gun.shootgun.len());
        self.player.create_items(self.round, "You".to_string());
        self.dealer.create_items(self.round, "Dealer".to_string());
    
        while self.player.lives > 0 && self.dealer.lives > 0 {
            if self.player.lives <= 3 && self.current_lvl == Levels::Level3 {
                println!("------------------");
            } else {
                println!("Your hp: {} | Dealer's hp: {}", self.player.lives, self.dealer.lives);
            }
            
            while self.player.nr_turns > 0 {
                if self.gun.shootgun.is_empty() {
                    println!("\nNo bullets. Reloading weapon...");
                    self.gun.shootgun = Gun::create(2, 8);
                    self.player.create_items(self.round, "You".to_string());
                    self.dealer.create_items(self.round, "Dealer".to_string());
                }
                self.player_turn();
                self.player.nr_turns -= 1;
                self.dealer.nr_turns += 1;
            }

            if self.dealer.lives == 0 {break;}

            while self.dealer.nr_turns > 0 {
                if self.gun.shootgun.is_empty() {
                    println!("\nNo bullets. Reloading weapon...");
                    self.gun.shootgun = Gun::create(2, 8);
                    self.player.create_items(self.round, "You".to_string());
                    self.dealer.create_items(self.round, "Dealer".to_string());
                }
                self.dealer_turn();
                self.dealer.nr_turns -= 1;
                self.player.nr_turns += 1;
            }

            if self.player.lives == 0 {break;}

            self.round += 1;
        }

        if self.player.lives > 0 {
            println!("You WIN!");
        } else {
            println!("You LOSE! Game Over!");
        }
    }

    fn player_turn(&mut self) {
        println!("\n=== Your turn ===");
        let mut choice;

        loop {
            println!("Actions: 1) Shoot dealer; 2) Shoot yourself; 3) Check items;");
            choice = utils::input();
            match choice.as_str() {
                "1" => {
                    println!("Shooting Dealer...");
                    self.gun.shoot(&mut self.dealer, "Dealer".to_string());
                    break;
                }
                "2" => {
                    println!("Shooting yourself...");
                    if self.gun.shootgun[0] == Bullet::Combat {
                        self.gun.shoot(&mut self.player, "You".to_string());
                        break;
                    } else {
                        self.gun.shoot(&mut self.player, "NoOne".to_string());
                        continue;
                    }
                        
                }
                "3" => {
                    self.use_item_menu();
                    continue;
                }
                _ => {
                    println!("Invalid option!");
                    continue;
                }
            }
        }
        println!("You made turn");
    }

    fn dealer_turn(&mut self) {
        let mut knows_bullet = false;
        let mut is_combat = false;

        println!("\n=== Dealer's turn ===");

        loop {
            //adrenalyne
            if self.dealer.items.adrenalyne > 0 {
                if self.dealer.lives < 5 && self.dealer.items.cigarettes < 5 && self.player.items.cigarettes > 0 {
                    self.dealer.items.cigarettes += 1;
                    self.player.items.cigarettes -= 1;
                    self.dealer.items.nr_items += 1;
                    self.player.items.nr_items -= 1;
                    self.dealer.items.adrenalyne -= 1;
                    println!("Dealer got your cigarettes");
                }
                continue;
            }

            if self.dealer.items.adrenalyne > 0 {
                if self.dealer.items.handcuffs < 5 && self.player.items.handcuffs > 0 {
                    self.dealer.items.handcuffs += 1;
                    self.player.items.handcuffs -= 1;
                    self.dealer.items.nr_items += 1;
                    self.player.items.nr_items -= 1;
                    self.dealer.items.adrenalyne -= 1;
                    println!("Dealer got your handcuffs");
                }
                continue;
            }

            if self.dealer.items.adrenalyne > 0 {
                if self.dealer.items.loupe < 5 && self.player.items.loupe > 0 {
                    self.dealer.items.loupe += 1;
                    self.player.items.loupe -= 1;
                    self.dealer.items.nr_items += 1;
                    self.player.items.nr_items -= 1;
                    self.dealer.items.adrenalyne -= 1;
                    println!("Dealer got your loupe");
                }
                continue;
            }
            
            //handcuffs
            if self.dealer.items.handcuffs > 0 {
                self.dealer.nr_turns += 1;
                self.dealer.items.handcuffs -= 1;
                self.dealer.items.nr_items -= 1;
                println!("Dealer used handcuffs!");
            }

            //loupe
            if !knows_bullet && self.dealer.items.loupe > 0 {
                println!("Dealer used loupe...");
                self.dealer.items.loupe -= 1;
                self.dealer.items.nr_items -= 1;
                knows_bullet = true;
                is_combat = self.gun.shootgun[0] == crate::gun::Bullet::Combat;
                continue;
            }

            //cigarettes
            let max_lives = match self.current_lvl {
                Levels::Level1 => 2,
                Levels::Level2 => 4,
                Levels::Level3 => 6,
            };
            if self.dealer.lives < max_lives && self.dealer.items.cigarettes > 0 {
                println!("Dealer used cigarettes...");
                self.dealer.add_live();
                self.dealer.items.nr_items -= 1;
                continue;
            }

            //adrenalyne
            if self.dealer.items.adrenalyne > 0 {
                if knows_bullet {
                    if is_combat && self.dealer.items.saw <= 0 && self.player.items.saw > 0 {
                        self.dealer.items.saw += 1;
                        self.player.items.saw -= 1;
                        self.dealer.items.nr_items += 1;
                        self.player.items.nr_items -= 1;
                        println!("Dealer got your saw");
                    }
                }
            }

            //saw
            if knows_bullet && is_combat && self.dealer.items.saw > 0 {
                println!("Dealer used saw (Double Damage)");
                self.gun.double_damage(&mut self.player, "Dealer".to_string());
                self.dealer.items.saw -= 1;
                self.dealer.items.nr_items -= 1;
                return;
            }

            if self.dealer.items.adrenalyne > 0 {
                if knows_bullet {
                    if !is_combat && self.dealer.items.invertor <= 0 && self.player.items.invertor > 0 {
                        self.dealer.items.invertor += 1;
                        self.player.items.invertor -= 1;
                        self.dealer.items.nr_items += 1;
                        self.player.items.nr_items -= 1;
                        println!("Dealer got your invertor");
                    }
                }
            }

            //invertor
            if knows_bullet && !is_combat && self.dealer.items.invertor > 0 {
                println!("Dealer uses invertor");
                self.gun.inverse_type();
                self.dealer.items.invertor -= 1;
                self.dealer.items.nr_items -= 1;
                is_combat = true;
                continue;
            }

            //adrenalyne
            if self.dealer.items.adrenalyne > 0 {
                if knows_bullet {
                    if !is_combat && self.dealer.items.beer <= 0 && self.player.items.beer > 0 {
                        self.dealer.items.beer += 1;
                        self.player.items.beer -= 1;
                        self.dealer.items.nr_items += 1;
                        self.player.items.nr_items -= 1;
                        println!("Dealer got your beer");
                    }
                }
            }

            //beer
            if knows_bullet && !is_combat && self.dealer.items.beer > 0 {
                println!("Dealer used beer... (Skip current bullet)");
                self.gun.skip_bullet();
                self.dealer.items.beer -= 1;
                self.dealer.items.nr_items -= 1;
                knows_bullet = false;
                continue;
            }
            
            //pills
            if self.dealer.lives < 4 && self.dealer.lives >= 1 && self.dealer.items.pills > 0 {
                println!("Dealer used pills...");
                self.dealer.pills();
                self.dealer.items.pills -= 1;
                self.dealer.items.nr_items -= 1;
            }

            //phone
            if self.dealer.items.phone > 0 && self.gun.shootgun.len() == 2 {
                println!("Dealer used phone...");
                self.dealer.items.phone -= 1;
                self.dealer.items.nr_items -= 1;
                knows_bullet = true;
                is_combat = self.gun.shootgun[0] == crate::gun::Bullet::Combat;
            }

            if knows_bullet {
                if is_combat {
                    println!("Dealer shooting you!");
                    self.gun.shoot(&mut self.player, "You".to_string());
                    return;
                } else {
                    println!("Dealer shoots himself...");
                    self.gun.shoot(&mut self.dealer, "NoOne".to_string());
                    knows_bullet = false;
                    continue;
                }
            } else {
                println!("Dealer shooting you...");
                self.gun.shoot(&mut self.player, "You".to_string());
            }

            break;
        }
    }

    fn use_item_menu(&mut self) {
        println!("Your items: 1) beer - {};\n 2) loupe - {};\n 3) saw - {};\n 4) cigarettes - {};\n
        5) handcuffs - {};\n 6) phone - {};\n 7) pills - {};\n 8) invertor - {};\n 9) adrenalyne - {};\n 0) go to shoot;\n", 
            self.player.items.beer, self.player.items.loupe, self.player.items.saw, self.player.items.cigarettes,
            self.player.items.handcuffs, self.player.items.phone, self.player.items.pills, self.player.items.invertor,
            self.player.items.adrenalyne
        );

        loop {
            println!("Use something (1-9 or 0 to exit from items): ");
            let choice = utils::input();
            match choice.as_str() {
                "1" => { 
                    if self.player.items.beer > 0 {
                        self.gun.skip_bullet();
                        self.player.items.beer -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used beer!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//beer
                "2" => {
                    if self.player.items.loupe > 0 {
                        self.gun.check_bullet();
                        self.player.items.loupe -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used loupe!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//loupe
                "3" => {
                    if self.player.items.saw > 0 {
                        self.gun.skip_bullet();
                        self.player.items.nr_items -= 1;
                        self.player.items.saw -= 1;
                        println!("You used saw!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//saw
                "4" => {
                    if self.player.items.cigarettes > 0 {
                        self.player.add_live();
                        self.player.items.cigarettes -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used cigarettes!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//cigarettes
                "5" => {
                    if self.player.items.handcuffs > 0 {
                        self.player.nr_turns += 1;
                        self.player.items.handcuffs -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used handcuffs!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//handcuffs
                "6" => {
                    if self.player.items.phone > 0 {
                        self.gun.to_know_rand_bullet();
                        self.player.items.phone -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used phone!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//phone
                "7" => {
                    if self.player.items.pills > 0 {
                        self.player.pills();
                        self.player.items.pills -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used pills!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//pills
                "8" => {
                    if self.player.items.invertor > 0 {
                        self.gun.inverse_type();
                        self.player.items.invertor -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used invertor!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//invertor
                "9" => {
                    if self.player.items.adrenalyne > 0 {
                        println!("{:?}", self.dealer.items);
                        println!("Choose an item: ");
                        loop {
                            let choice = utils::input();
                            match choice.as_str() {
                                "beer" => {
                                    if self.player.items.beer > 4 {
                                        println!("You already have max count of beer");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.beer > 0 {
                                        self.dealer.items.beer -= 1;
                                        self.player.items.beer += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took beer");
                                        break;
                                    } else {
                                        println!("Dealed don't have beer");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "loupe" => {
                                    if self.player.items.loupe > 4 {
                                        println!("You already have max count of loupe");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.loupe > 0 {
                                        self.dealer.items.loupe -= 1;
                                        self.player.items.loupe += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took loupe");
                                        break;
                                    } else {
                                        println!("Dealed don't have loupe");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "cigarettes" => {
                                    if self.player.items.cigarettes > 4 {
                                        println!("You already have max count of cigarettes");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.cigarettes > 0 {
                                        self.dealer.items.cigarettes -= 1;
                                        self.player.items.cigarettes += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took cigarettes");
                                        break;
                                    } else {
                                        println!("Dealed don't have cigarettes");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "saw" => {
                                    if self.player.items.saw > 4 {
                                        println!("You already have max count of saw");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.saw > 0 {
                                        self.dealer.items.saw -= 1;
                                        self.player.items.saw += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took saw");
                                        break;
                                    } else {
                                        println!("Dealed don't have saw");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "handcuffs" => {
                                    if self.player.items.handcuffs > 4 {
                                        println!("You already have max count of handcuffs");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.handcuffs > 0 {
                                        self.dealer.items.handcuffs -= 1;
                                        self.player.items.handcuffs += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took handcuffs");
                                        break;
                                    } else {
                                        println!("Dealed don't have handcuffs");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "phone" => {
                                    if self.player.items.phone > 4 {
                                        println!("You already have max count of phone");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.phone > 0 {
                                        self.dealer.items.phone -= 1;
                                        self.player.items.phone += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took phone");
                                        break;
                                    } else {
                                        println!("Dealed don't have phone");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "pills" => {
                                    if self.player.items.pills > 4 {
                                        println!("You already have max count of pills");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.pills > 0 {
                                        self.dealer.items.pills -= 1;
                                        self.player.items.pills += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took pills");
                                        break;
                                    } else {
                                        println!("Dealed don't have pills");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "invertor" => {
                                    if self.player.items.invertor > 4 {
                                        println!("You already have max count of invertor");
                                        println!("Choose other item");
                                        continue;
                                    }
                                    if self.dealer.items.invertor > 0 {
                                        self.dealer.items.invertor -= 1;
                                        self.player.items.invertor += 1;
                                        self.dealer.items.nr_items -= 1;
                                        self.player.items.nr_items += 1;
                                        println!("You took invertor");
                                        break;
                                    } else {
                                        println!("Dealed don't have invertor");
                                        println!("Choose other item");
                                        continue;
                                    }
                                },
                                "adrenalyne" => {
                                    println!("You cant take adrenalyne");
                                    println!("Choose other item");
                                    continue;
                                },
                                _ => {
                                    println!("This item does not exist");
                                    println!("Choose other item");
                                    continue;
                                },
                            }
                        }
                        self.player.items.adrenalyne -= 1;
                        self.player.items.nr_items -= 1;
                        println!("You used adrenalyne!");
                    } else {
                        println!("You dont have this item!");
                    }
                },//adrenalyne
                "0" => {
                    println!("Go to shoot");
                    break;
                },
                _ => {
                    println!("Incorect! Try a number from 0 to 9");
                },
            }
        }
    }
}