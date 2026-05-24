import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        ArrayList<MaturatorMustecios> lista = new ArrayList<MaturatorMustecios>();
        for (int i =0; i<10; i++) {
            lista.add(new MaturatorMustecios());
        }
        for (MaturatorMustecios m: lista) {
            m.printMaturatorMustacios();
        }
        Scanner sc = new Scanner(System.in);

        // 1. Primul Maturator (Implicit)
        System.out.println("\n--- Creare Maturator 1 (Implicit) ---");
        MaturatorMustecios m1 = new MaturatorMustecios();
        m1.printMaturatorMustacios();

        int nouNrMaturi = MaturatorMustecios.citesteNrMaturi(sc);
        m1.setNrMaturi(nouNrMaturi);

        // 2. Modificarea Datelor Maturatorului 1
        // Folosește: citireIndexMatura (Tip 1) și citesteIntLenes (Tip 2)
        if (m1.sklad.size() > 0) {
            System.out.println("\n--- Modificarea Datelor Maturatorului 1:");
            m1.setInitialData(sc);
            m1.printMaturatorMustacios();
        }

        // 3. Pregătirea datelor pentru al doilea Maturator (Cu Parametri)
        System.out.println("\n--- Creare Maturator 2 (Cu Parametri) ---");

        // Citirea boolean-ului pentru licență (Tip 4)
        boolean licenta = MaturatorMustecios.citesteBooleanLicenta(sc);

        // Citirea numărului de mături (Tip 3 - Try în Try)
        int nrMaturi = MaturatorMustecios.citesteNrMaturi(sc);

        // Crearea Maturatorului 2
        MaturatorMustecios m2 = new MaturatorMustecios(licenta, nrMaturi);
        m2.printMaturatorMustacios();

        // 4. Modificarea Datelor Maturatorului 2
        // Folosește: citireIndexMatura (Tip 1) și citesteIntLenes (Tip 2)
        if (m2.sklad.size() > 0) {
            System.out.println("\n--- Modificarea Datelor Maturatorului 2 (Foloseste Tip 1, 2, 1) ---");
            m2.setInitialData(sc);
            m2.printMaturatorMustacios();
        } else {
            new ExceptieMatura("Maturatorul 2 nu are mături de modificat.").prelucrare();
        }

        System.out.println("\nTotal Maturatori creati: " + MaturatorMustecios.maturatori);

        sc.close();
    }
}