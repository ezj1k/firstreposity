import java.util.Random;

public final class Defuser extends CounterTerrorist implements BombPlanted {
    boolean hasDefuseKit;

    public Defuser() {
        super();
        randomize();
    }

    public Defuser(String nickname, int hp, String gun, int armor, boolean hasDefuseKit) {
        super(nickname, hp, gun, armor);
        this.hasDefuseKit = hasDefuseKit;
    }

    public void shooting() {
        System.out.println(nickname+" shoots with "+gun+" and tries to defuse bomb");
    }

    public  void interactionBomb() {
        if (hasDefuseKit) {
            System.out.println(nickname+" will defuse bomb in 5 seconds");
        } else {
            System.out.println(nickname+" will defuse bomb in 10 seconds");
        }
    }

    public void println() {
        super.println();
        System.out.println("hasDefuseKit: " + hasDefuseKit);
    }

    public void detonate() {
        if(hasDefuseKit) {
            System.out.println(nickname+" has defuse kit, adn he dont lost 400$.");
        } else {
            System.out.println(nickname+" bought a defuse kits and pay 400$.");
            hasDefuseKit = true;
        }
    }

    public void randomize() {
        super.randomize();
        this.hasDefuseKit = new Random().nextBoolean();
        this.gun = "MP5-SD";
    }

    public void readData(java.util.Scanner sc) {
        super.readData(sc);
        System.out.print("Has defuse kit (true/false): ");
        this.hasDefuseKit = sc.nextBoolean();
        sc.nextLine();
    }
}
