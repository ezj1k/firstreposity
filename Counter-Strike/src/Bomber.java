import java.util.Random;

public class Bomber extends Terrorist {
    int explosionPower;

    public Bomber() {
        super();
        randomize();
    }

    public Bomber(String nickname, int hp, String gun, boolean bombPlanted, int explosionPower) {
        super(nickname,hp,gun,bombPlanted);
        this.explosionPower = explosionPower;
    }

    public void shooting() {
        System.out.println(nickname+" shoots with "+ gun+" before detonating!");
    }

    public void println() {
        super.println();
        System.out.println("Explosion Power: "+explosionPower);
    }

    public void detonate() {
        if(bombPlanted) {
            System.out.println(nickname+" plants bomb and it will detonate with power "+explosionPower);
        } else {
            System.out.println(nickname+" dont plants bomb to detonate.");
        }
    }

    public void randomize() {
        super.randomize();
        this.explosionPower = new Random().nextInt(100)+50;
    }
}
