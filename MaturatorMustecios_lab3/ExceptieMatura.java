import java.io.IOException;

public class ExceptieMatura extends Exception {
    Object eroare;

    // Constructor unic conform cerinței
    ExceptieMatura(Object eroare) {
        this.eroare = eroare;
    }

    void prelucrare() {
        if (eroare instanceof Integer) {
            int val = (Integer) eroare;
            if (val < 0) {
                System.out.println("Eroare: Valoare negativă pentru numărul de mături (" + val + ").");
            } else {
                System.out.println("Eroare: Număr de mături incorect (" + val + ").");
            }
        } else if (eroare instanceof String) {
            String mesaj = (String) eroare;
            System.out.println("Eroare: " + mesaj);
        } else if (eroare instanceof NumberFormatException) {
            System.out.println("Eroare: Valoarea introdusă nu este un număr valid!");
        } else if (eroare instanceof NegativeArraySizeException) {
            System.out.println("Eroare: Dimensiune negativă a tabloului!");
        } else if (eroare instanceof IOException) {
            System.out.println("Eroare IO: Problemă la citire/scriere!");
        } else if (eroare instanceof Exception) {
            System.out.println("Eroare necunoscută de tip: " + eroare.getClass().getSimpleName());
        } else {
            System.out.println("Eroare necunoscută!");
        }
    }
}