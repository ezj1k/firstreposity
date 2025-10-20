import java.util.Random;

public class Man extends Human{
    double money;
    boolean orientation;

    public Man() {
        super();
        money = (new Random().nextDouble()*10000);
        orientation = (new Random().nextDouble()>0.5)?true:false;
    }

    public Man(String x, int y, double z) {
        super(x,y);
        if (z>0.0) {
        money=z;
        orientation = (new Random().nextDouble()>0.5) ;
        }
    }

    public void println() {
        super.println();
        System.out.println("Money: "+money +"; orientation: " + orientation);
    }

    public void drive() {
        if(money>100) {
            System.out.println("i drive car");
        } else {
            System.out.println("i drive skate");
        }
    }

    void sayHi() {
        if (orientation) {
            System.out.println("Privet");
        } else {
            System.out.println("4moki");
        }
    }

    public void WineDay() {
        name = "?";
        money/=2;
        orientation=!orientation;
    }
}
