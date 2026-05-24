import java.util.Random;

public class Terrorist extends Player implements BombPlanted {
    boolean bombPlanted;

    public Terrorist() {
        super();
        randomize();
    }

    public Terrorist(String nickname, int hp, String gun, boolean bombPlanted) {
        super(nickname, hp, gun);
        this.bombPlanted = bombPlanted;
    }

    public void shooting() {
        System.out.println(nickname+" shoots to attack the site with "+gun+"!");
    }

    public void interactionBomb() {
        if (bombPlanted) {
            System.out.println(nickname+" is defending the bomb!");
        } else {
            System.out.println(nickname+" has not planted the bomb yet.");
        }
    }

    public void println() {
        super.println();
        System.out.println("Team: Terrorists: Bomb planted - "+bombPlanted);
    }

    public void changeGun(String newGun) {
        gun = newGun;
        System.out.println(nickname+" changed gun to "+ gun);
    }

    public void randomize() {
        this.bombPlanted = new Random().nextBoolean();
        this.gun = "AK-47";
    }

    public void readData(java.util.Scanner sc) {
        super.readData(sc);
        System.out.print("Is bomb planted (true/false): ");
        this.bombPlanted = sc.nextBoolean();
        sc.nextLine();
    }
}
