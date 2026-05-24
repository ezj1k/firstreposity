import java.util.Random;

public class Matura {
     double maturaPrice;
     double maturaCalitate;

     Matura() {
         Random rnd = new Random();
         double maturaCalitateTemp = rnd.nextDouble(5.0)+1.0;
         maturaCalitate = Math.round(maturaCalitateTemp * 100) / 100.0f;
         double maturaPriceTemp = rnd.nextDouble(50.0)+30.0;
         maturaPrice = Math.round(maturaPriceTemp * 100) / 100.0f;
     }

    void shortPrint() {
        System.out.printf("Matura coasta %.2f, si ea este de calitate: %.2f;%n", maturaPrice, maturaCalitate);
    }
}
