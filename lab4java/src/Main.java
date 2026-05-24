import java.util.Random;

public class Main {
    static int rows, cols;
    static Matura raft[][];
    static Matura bestOfTheBest[];
    static Matura theLowestPrice[];

    static{
        rows = (new Random()).nextInt(10)+1;
        cols = (new Random()).nextInt(10)+1;
        raft = new Matura[rows][cols];
        for (int i = 0; i<rows; i++) {
            for(int j=0; j<cols;j++) {
                raft[i][j] = new Matura();
            }
        }

        bestOfTheBest = new Matura[rows];
        theLowestPrice = new Matura[cols];
    }

    public static void main(String []a) {

        for (int i = 0; i<rows; i++) {
            for (int j=0; j<cols; j++) {
                raft[i][j].shortPrint();
            }
        }

        for (int i =0; i<rows; i++) {
            (new RowsCalitate(i)).start();
        }

        for (int i =0; i<cols; i++) {
            (new Thread(new ColsBestPrice(i))).start();
        }

        try {Thread.sleep(1000);}
        catch (Exception e){
            System.out.println("eroare");
        }

        //afisam frumoase si bune

        Matura ceaMaiBunaDinIeftine = bestOfTheBest[0];
        for (int i = 1; i<bestOfTheBest.length; i++) {
            if(ceaMaiBunaDinIeftine.maturaPrice > bestOfTheBest[i].maturaPrice) {
                ceaMaiBunaDinIeftine = bestOfTheBest[i];
            }
        }

        Matura ceaMaiIeftinaDinBune = theLowestPrice[0];
        for (int i = 1; i<Main.theLowestPrice.length; i++) {
            if(ceaMaiIeftinaDinBune.maturaCalitate > theLowestPrice[i].maturaCalitate) {
                ceaMaiIeftinaDinBune = theLowestPrice[i];
            }
        }

        //afisam ce am aflat
        System.out.println("-------------------");
        for (int i = 1; i<bestOfTheBest.length; i++) {
            bestOfTheBest[i].shortPrint();
        }
        for (int i = 1; i<theLowestPrice.length; i++) {
            theLowestPrice[i].shortPrint();
        }

        System.out.print("Cea Mai Ieftina Din Bune: ");
        ceaMaiIeftinaDinBune.shortPrint();
        System.out.print("Cea Mai Buna Din Ieftine: ");
        ceaMaiBunaDinIeftine.shortPrint();
    }
}
