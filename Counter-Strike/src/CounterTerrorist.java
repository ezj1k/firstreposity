import java.util.Random;

public class CounterTerrorist extends Player {
    int armor;

    public CounterTerrorist() {
        super();
        randomize();
    }

    public CounterTerrorist(String nickname, int hp, String gun, int armor) {
        super(nickname, hp, gun);
        this.armor = armor;
    }

    public void shooting() {
        System.out.println(nickname + " shoots with " + gun + " to defend the site!");
    }

    public void println() {
        super.println();
        System.out.println("Team: Counter-Terrorist: Armor: " + armor);
    }

    public void buyNewArmor() {
        armor = 100;
        System.out.println(nickname + " bought new armor and now it has 100.");
    }

    public void randomize() {
        super.randomize();
        this.armor = new Random().nextInt(101);
        this.gun = "M4A1";
    }
}
