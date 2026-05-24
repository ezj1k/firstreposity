import java.util.Scanner;
import java.util.ArrayList;

public class MaturatorMustecios {
    static int maturatori = 0;
    String nume;
    boolean areLicenta;
    ArrayList<Maturi> sklad = new ArrayList<>();

    MaturatorMustecios() {
        maturatori++;
        nume = "Vanea";
        areLicenta = false;
        // Folosire for-loop
        for (int i = 0; i < 5; i++) {
            sklad.add(new Maturi());
        }
    }

    MaturatorMustecios(boolean areSauNu, int nrMaturi) {
        maturatori++;
        nume = "Pasha";
        areLicenta = areSauNu;
        // Folosire for-loop
        for (int i = 0; i < nrMaturi; i++) {
            sklad.add(new Maturi());
        }
    }

    void printMaturatorMustacios() {
        System.out.println("\n--- Detalii Maturator ---");
        System.out.println("Nume: " + nume + ", Licență: " + areLicenta + ", Nr. Mături: " + sklad.size());
        if (!sklad.isEmpty()) {
            System.out.println("Mături:");
            // Folosire for-each
            int index = 1;
            for (Maturi m : sklad) {
                System.out.print("Mătura " + index++ + ": ");
                m.printMaturi();
            }
        } else {
            System.out.println("Nu are mături :c");
        }
    }

    // --------------------------------------------------
    // Metode de citire cu do-while + ExceptieMatura
    // --------------------------------------------------

    // TIP 1: Citire, Prindere și Prelucrare Imediată (similar cu citireIndexMatura)
    public static int citireIndexMatura(Scanner sc, int nrTotalMaturi) {
        boolean flag;
        int indexAles = -1;
        do {
            flag = false;
            System.out.print("Introdu numărul măturii (1-" + nrTotalMaturi + "): ");
            try {
                String linie = sc.nextLine();
                indexAles = Integer.parseInt(linie);
                if (indexAles < 1 || indexAles > nrTotalMaturi)
                    throw new ExceptieMatura("Indice invalid: " + indexAles + "!");
            } catch (ExceptieMatura | NumberFormatException e) {
                flag = true;
                if (e instanceof ExceptieMatura) {
                    ((ExceptieMatura) e).prelucrare();
                } else {
                    new ExceptieMatura(e).prelucrare(); // Împachetarea NumberFormatException
                }
            }
        } while (flag);
        return indexAles;
    }

    //este la fel tip 1 doar ca pentru densitate
    public static int citesteIntValidat(Scanner sc, String mesaj, int limitaMax) {
        boolean flag;
        int valoareAles = -1;
        do {
            flag = false;
            System.out.print(mesaj + " (1-" + limitaMax + "): ");
            try {
                String linie = sc.nextLine();
                valoareAles = Integer.parseInt(linie);
                if (valoareAles < 1 || valoareAles > limitaMax)
                    throw new ExceptieMatura("Valoare invalidă: " + valoareAles + "!"); // Trimitere String ca Object
            } catch (ExceptieMatura | NumberFormatException e) {
                flag = true;
                if (e instanceof ExceptieMatura) {
                    ((ExceptieMatura) e).prelucrare();
                } else {
                    new ExceptieMatura(e).prelucrare(); // Împachetarea NumberFormatException
                }
            }
        } while (flag);
        return valoareAles;
    }

    // TIP 2: Metoda Leneșă - Aruncă Excepția mai Departe
    public static int citesteIntLenes(Scanner sc, String mesaj) {
        boolean flag;
        int valoare = 0;
        do {
            flag = false;
            System.out.print(mesaj);
            try {
                String linie = sc.nextLine();
                valoare = Integer.parseInt(linie);
                if (valoare < 1 || valoare > 5) throw new ExceptieMatura("Valoare (1-5) incorectă!");
            } catch (ExceptieMatura e) {
                flag = true;
                e.prelucrare();
            } catch (NumberFormatException e) {
                flag = true;
                new ExceptieMatura(e).prelucrare();
            }
        } while (flag);
        return valoare;
    }

    // TIP 3: Try în Try (Prinde excepția din interior și o aruncă/împachetează în exterior)
    public static int citesteNrMaturi(Scanner sc) {
        boolean flag;
        int nr = 0;
        do {
            flag = false;
            System.out.print("Introdu nr de mături: ");
            try {
                try {
                    nr = Integer.parseInt(sc.nextLine());
                    if (nr < 0) throw new ExceptieMatura(nr); // Trimitere int ca Object
                } catch (NumberFormatException e) {
                    throw new ExceptieMatura(e); // Împachetează NFE într-o ExceptieMatura
                }
            } catch (ExceptieMatura e) {
                flag = true;
                e.prelucrare();
            }
        } while (flag);
        return nr;
    }

    // Similar Tipului 1: Prinde și Prelucrarea Imediată
    public static boolean citesteBooleanLicenta(Scanner sc) {
        boolean flag;
        boolean licenta = false;
        do {
            flag = false;
            System.out.print("Are licență? (true/false): ");
            try {
                String linie = sc.nextLine().trim().toLowerCase();
                if (linie.equals("true")) {
                    licenta = true;
                } else if (linie.equals("false")) {
                    licenta = false;
                } else {
                    throw new ExceptieMatura("Valoarea licenței trebuie să fie 'true' sau 'false'!");
                }
            } catch (ExceptieMatura e) {
                flag = true;
                e.prelucrare();
            } catch (Exception e) {
                flag = true;
                new ExceptieMatura("Eroare necunoscută la citirea licenței.").prelucrare();
            }
        } while (flag);
        return licenta;
    }

    // --------------------------------------------------
    // Metoda principală care folosește toate tipurile de prelucrare
    // --------------------------------------------------
    public void setInitialData(Scanner sc) {
        System.out.println("\n--- Setare Date Maturator cu Prelucrări ---");

        if (this.sklad.isEmpty()) {
            new ExceptieMatura("Nu există mături de modificat!").prelucrare();
            return;
        }

        System.out.println("\nLista de mături curente:");
        for (int i = 0; i < this.sklad.size(); i++) {
            System.out.print("Mătura " + (i + 1) + ": ");
            this.sklad.get(i).printMaturi();
        }

        // Citirea Indexului - Tipul 1
        int indexAles = citireIndexMatura(sc, this.sklad.size());
        int indexLista = indexAles - 1;

        // Citirea Coeficientului - Tipul 2 (Metoda Leneșă)
        int coef = citesteIntLenes(sc, "Introdu coeficientul de uzură (1-5): ");
        this.sklad.get(indexLista).setCoef(coef);

        // Citirea Densității - Tipul 1.1 (folosește noua metodă cu mesaj corect)
        int densitate = citesteIntValidat(sc, "Introdu densitatea", 5);
        this.sklad.get(indexLista).setDensitate(densitate);

        System.out.println("--- Setare Date Finalizată ---");
    }

    void setNrMaturi(int nrMaturi) {
        if (nrMaturi>0 && nrMaturi != sklad.size()) {
            int dif = nrMaturi - sklad.size();
            if (dif > 0) {
                for (int i = 0; i < dif; i++) {
                    sklad.add(new Maturi());
                }
            } else {
                for (int i = dif; i < 0; i++) {
                    sklad.remove(0);
                }
            }
        }
    }
}