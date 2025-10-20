import java.util.Random;

public abstract class Player implements Afisare {
    String nickname;
    int hp;
    String gun;
    static int totalPlayers = 0;
    final static int MAX_HP = 100;

    public Player() {
        randomize();
        totalPlayers++;
        nickname = "Player" + totalPlayers;
        gun = "Pistol";
    }

    public Player(String nickname, int hp, String gun) {
        this.nickname = nickname;
        this.hp = hp;
        this.gun = gun;
        totalPlayers++;
    }

    public void println() {
        System.out.println("nickname: "+nickname+"; hp: "+hp+"; gun: "+gun+";");
    }

    public void readData(java.util.Scanner sc) {
        System.out.println("Introduce nickname: ");
        this.nickname = sc.nextLine();
        System.out.println("Introduce HP (1-100): ");
        this.hp = sc.nextInt();
        System.out.println("Introduce Gun: ");
        this.gun = sc.nextLine();
    }

    public void randomize() {
        this.hp = new Random().nextInt(MAX_HP) + 1;
    }

    public final void sayHello() {
        System.out.println(nickname + " says: Let's go!");
    }

    public abstract void shooting();

    public static int getTotalPlayersCount() {
        return totalPlayers;
    }
}
