const heal=document.getElementById("heal");
const dodge=document.getElementById("dodge");
const attack=document.getElementById("attack");
const log=document.getElementById("log");

class Player{
  constructor(){
    this.attack=20;
    this.health=100;
  }
  
  attackp(monster) {
    log.innerHTML=""
    monster.health -= this.attack;
    
    if (monster.health >= 0) {
      this.health -= monster.attack;
    }
    if (this.health <= 0) {
      log.innerHTML += `<p>Player is dead!</p>`;
      this.health=0;
      dissablebuttons()
    } else if (monster.health <= 0) {
      log.innerHTML += `<p>${monster.name} is dead!</p>`;
      currentMonsterIndex++;
    } else {
      log.innerHTML += `<p>${monster.name} has ${monster.health} hp!</p>`;
      log.innerHTML += `<p>Player has ${this.health} hp!</p>`;
    }
    update();
  }

  heal(){
    if(this.health<90) {
    this.health+=10;
    log.innerHTML+=`<p>Player has ${this.health} hp!</p>`;
    }
    else if(this.health>89 && this.health<100){
      this.health=100;
      log.innerHTML+=`<p>Player has ${this.health} hp!</p>`;
    }
    if(this.health===100){
      log.innerHTML+=`<p>HP is full!</p>`;
    }
    update();
  }

  dodge(monster){
    if(Math.random()<0.5){
      log.innerHTML+=`<p>Player dodge the damage!</p>`;
    }
    else{
      this.health-=monster.attack;
      log.innerHTML+=`<p>Player has ${this.health} hp</p>`;
    }
    update();
  }
}

class Monsters{
  constructor(name,attack,health) {
    this.name=name;
    this.attack=attack;
    this.health=health;
  }
}

function dissablebuttons(){
attack.disabled=true;
heal.disabled=true;
dodge.disabled=true;
}

function update() {
  document.getElementById("health1").innerText = player.health;
  
  if (currentMonsterIndex < monsters.length) {
    const currentMonster = monsters[currentMonsterIndex];
    document.getElementById("name2").innerText = currentMonster.name;
    document.getElementById("health2").innerText = currentMonster.health;
    document.getElementById("attack2").innerText = currentMonster.attack;
  } else {
    document.getElementById("name2").innerText = "No more monsters!";
    document.getElementById("health2").innerText = "-";
    document.getElementById("attack2").innerText = "-";
    log.innerHTML += "<p>You defeated all the monsters!</p>";
    dissablebuttons()
  }
}

let player = new Player;
let monster1 = new Monsters("Skeleton", 20, 50);
let monster2 = new Monsters("Zombie", 10, 100);
let monster3 = new Monsters("Dragon", 50, 50);
const monsters = [monster1,monster2,monster3];
let currentMonsterIndex = 0;

attack.addEventListener("click", ()=>player.attackp(monsters[currentMonsterIndex]));
heal.addEventListener("click", ()=>player.heal());
dodge.addEventListener("click",()=> player.dodge(monsters[currentMonsterIndex]));