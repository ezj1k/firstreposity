import java.util.Random;

public class Woman extends Human{
    float beauty; //0..1
    byte beachness;

    public Woman() {
        super();
        beauty = new Random().nextFloat();
        beachness = (byte)(new Random().nextInt(127));
    }

    public Woman(String a, int b, float c, byte d) {
        super(a,b);
        if (c>=0.0 && c<=1.0) {
            beauty = c;
        } else {
            System.out.println("error number");
        }
        if (d>0) {
            beachness = d;
        } else {
            System.out.println("error number");
        }
    }

    public void makeUp() {
        beauty = 0f;
        System.out.println("nu va voi folosi machiaj niciodata!");
    }

    public void println() {
        super.println();
        System.out.println("Beauty: "+beauty+"; Beachness: "+ beachness);
    }

    void sayHi() {
        if (beachness<100) {
            System.out.println("Privet");
        } else {
            System.out.println("dina");
        }
    }

    public void WineDay() {
        beauty = 0.1f;
        beachness = (byte)120;
    }
}
