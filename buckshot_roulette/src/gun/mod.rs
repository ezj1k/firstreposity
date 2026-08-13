use rand::prelude::*;
use rand::seq::IteratorRandom;
use strum::{EnumIter, IntoEnumIterator};

use crate::player::{Player};

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Bullet {
    Combat,
    Blank,
}

pub struct Gun {
    pub shootgun: Vec<Bullet>,
}

impl Gun {
    pub fn create(range1: u8, range2: u8) -> Vec<Bullet> {
        let mut rng = rand::rng();
        let nr_bullets = rng.random_range(range1..=range2);

        let mut bullets_vec = Vec::new();

        bullets_vec.push(Bullet::Combat);
        bullets_vec.push(Bullet::Blank);

        if nr_bullets > 2 {
            for _ in 2..nr_bullets {
                let bullet_type = Bullet::iter().choose(&mut rng).unwrap();
                bullets_vec.push(bullet_type);
            }
        }

        
        bullets_vec.shuffle(&mut rng);

        bullets_vec
    }

    pub fn shoot(&mut self, player: &mut Player, who: String) {
        if self.shootgun[0] == Bullet::Combat {
            player.lives -= 1;
            println!("{who} deal 1 damage!");
        } else {
            println!("Bullet was blank");
        }
        self.shootgun.remove(0);
    }
    //shoot
    
    pub fn double_damage(&mut self, player: &mut Player, who: String) {
        if self.shootgun[0] == Bullet::Combat {
            player.lives -= 2;
            println!("{who} deal 2 damage!");
        } else {
            println!("Bullet was blank");
        }
        self.shootgun.remove(0);
    }
    //saw

    pub fn skip_bullet(&mut self) {
        self.shootgun.remove(0);
    }
    //beer

    pub fn check_bullet(&self) {
        println!("Next bullet is: {:?}", self.shootgun.first().unwrap());
    }
    //loupe

    pub fn inverse_type(&mut self) {
        if self.shootgun[0] == Bullet::Combat {
            self.shootgun[0] = Bullet::Blank;
        } else {
            self.shootgun[0] = Bullet::Combat;
        }
    }
    //invertor

    pub fn to_know_rand_bullet(&mut self) {
        let mut rng = rand::rng();
        let rand_bullet = rng.random_range(1..=self.shootgun.len());
        println!("{}th bullet is: {:?}", rand_bullet, self.shootgun[rand_bullet-1]);
    } //phone
}

//beer - skip bullet 
//loupe - check current bullet 
//cigarettes - +1hp 
//saw - -2hp 
//invertor - change current bullet type 
//pills - 50% +2hp or -1hp 
//phone - check 1 random bullet except first 
//handcuffs - skip turn 
//adrenalyne - take 1 item from opponent and use it except adrenalyne -------------

//ne viju nr_items -= 1 u predmetov ot gun