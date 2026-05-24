import java.util.Random;

public class Maturi {
    int densitate; // 1-5
    int coef; // 1-5, uzură

    Maturi() {
        Random rnd = new Random();
        densitate = rnd.nextInt(5) + 1;
        coef = rnd.nextInt(5) + 1;
    }

    void printMaturi() {
        System.out.println("Matura cu densitatea " + densitate + "/5, este uzata la " + coef + "/5");
    }

    // Getters and Setters
    public int getCoef() { return coef; }
    public void setCoef(int coef) { this.coef = coef; }

    public int getDensitate() { return densitate; }
    public void setDensitate(int densitate) { this.densitate = densitate; }
}