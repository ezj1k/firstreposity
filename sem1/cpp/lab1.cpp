#include <iostream>
#include <algorithm>

using namespace std;

class Apartament {
private:
    int pret, numarul_odaii, suprafata;
    string adresa;
public:
    Apartament() {
        pret = 0;
        numarul_odaii = 0;
        suprafata = 0;
    }

    Apartament(const Apartament& a) {
        pret = a.pret;
        numarul_odaii = a.numarul_odaii;
        suprafata = a.suprafata;
        adresa = a.adresa;
    }

    Apartament(int a, int b, int c, string d) {
        pret = a;
        numarul_odaii = b;
        suprafata = c;
        adresa = d;
    }

    ~Apartament() {
        clog << "Adresa: " << adresa << "; Numarul odaii: " << numarul_odaii << "; Suprafata: " << suprafata << "; Pretul: " << pret << ";" << endl;
    }

    void setPret(int pret) { this->pret = pret; }
    void setNumarulOdaii(int numarul_odaii) { this->numarul_odaii = numarul_odaii; }
    void setSuprafata(int suprafata) { this->suprafata = suprafata; }
    void setAdresa(string adresa) { this->adresa = adresa; }

    int getPret() const { return pret; }
    int getNumarulOdaii() const { return numarul_odaii; }
    int getSuprafata() const { return suprafata; }
    string getAdresa() const { return adresa; }

    void afiseaza() const {
        cout << "Pretul: " << getPret() << endl;
        cout << "Numarul de odaie: " << getNumarulOdaii() << endl;
        cout << "Suprafata: " << getSuprafata() << endl;
        cout << "Adresa: " << getAdresa() << endl;
    }

    void introducere() {
        cout << "Introduceti adresa: ";
        cin >> adresa;
        cout << "Introduceti numarul de odăi: ";
        cin >> numarul_odaii;
        cout << "Introduceti suprafata (in metri patrati): ";
        cin >> suprafata;
        cout << "Introduceti pretul: ";
        cin >> pret;
    }
};

// Functie de comparare pentru sortare (ordonare alfabetica dupa adresa)
bool compareAdresa(const Apartament* a1, const Apartament* a2) {
    return a1->getAdresa() < a2->getAdresa();
}

void meniu() {
    const int MAX_APARTAMENTE = 100;  // Dimensiunea maxima a tabloului de apartamente
    Apartament* apartamente[MAX_APARTAMENTE];
    int numar_apartamente = 0;  // Urmarim numarul de apartamente din tablou

    for (; ; ) {
        cout << "\nMeniu:\n";
        cout << "1. citirea de la tastatura a valorilor membrilor private a obiectelor tabloului definit\n";
        cout << "2. afișarea obiectelor tabloului la ecran\n";
        cout << "3. sortarea obiectelor în ordine alfabetică după adresa\n";
        cout << "4. afișarea la ecran a obiectelor pentru care se cunoaște că prețul este o mărime z (z se citește de la tastatură)\n";
        cout << "5. se adaugă un obiect nou pe poziția k în tabloul de obiecte\n";
        cout << "6. se șterge obiectul pentru care se cunoaște că suprafața este o mărime p (p se citește de la tastatură)\n";
        cout << "7. Stop\n";
        cout << "Alegeti optiunea: ";

        int optiune;
        cin >> optiune;

        switch (optiune) {
        case 1: {
            int n;
            cout << "Introduceti numarul de apartamente (maxim " << MAX_APARTAMENTE << "): ";
            cin >> n;

            // Verificăm că nu depășim dimensiunea maximă a tabloului
            if (n < 0 || numar_apartamente + n > MAX_APARTAMENTE) {
                int diff = MAX_APARTAMENTE - numar_apartamente;
                cout << "Numar invalid de apartamente. Puteti adauga maxim " << diff << " apartamente.\n";
                n = diff;
            }

            for (int i = 0; i < n; i++) {
                apartamente[numar_apartamente] = new Apartament;
                apartamente[numar_apartamente]->introducere();
                numar_apartamente++;
            }
            break;
        }

        case 2:
            for (int i = 0; i < numar_apartamente; i++) {
                apartamente[i]->afiseaza();
                cout << "-------------" << endl;
            }
            break;

        case 3:
        // Sortăm apartamentele după adresă
        sort(apartamente, &apartamente[numar_apartamente], compareAdresa);
        
        // Afișăm apartamentele după sortare
        for (int i = 0; i < numar_apartamente; i++) {
            apartamente[i]->afiseaza();
            cout << "-------------" << endl;
        }
        cout << "Apartamentele au fost sortate dupa adresa.\n";
        break;

        case 4: {
            int z;
            cout << "Introduceti pretul (z): ";
            cin >> z;
            for (int i = 0; i < numar_apartamente; i++) {
                if (apartamente[i]->getPret() == z) {
                    apartamente[i]->afiseaza();
                    cout << "-------------" << endl;
                }
            }
            break;
        }

        case 5: {
            int k;
            cout << "Introduceti pozitia (k) pentru adaugarea unui nou apartament: ";
            cin >> k;

            // Verificăm dacă poziția este validă
            if (k >= 0 && k <= numar_apartamente && k < MAX_APARTAMENTE) {
                // Mutăm elementele pentru a face loc unui nou apartament
                for (int i = numar_apartamente; i > k; --i) {
                    apartamente[i] = apartamente[i - 1];
                }
                
                apartamente[k] = new Apartament;
                apartamente[k]->introducere();
                numar_apartamente++;
            } else {
                cout << "Pozitie invalida.\n";
            }
            break;
        }

        case 6: {
            float p;
            cout << "Introduceti suprafata (p) pentru care doriti sa stergeti apartamentul: ";
            cin >> p;

            bool gasit = false;
            for (int i = 0; i < numar_apartamente; ++i) {
                if (apartamente[i]->getSuprafata() == p) {
                    delete apartamente[i];
                    // Mutăm elementele pentru a "șterge" apartamentul
                    for (int j = i; j < numar_apartamente - 1; ++j) {
                        apartamente[j] = apartamente[j + 1];
                    }
                    numar_apartamente--;
                    gasit = true;
                    break;
                }
            }
            if (!gasit) {
                cout << "Nu s-a gasit apartamentul cu suprafata specificata.\n";
            }
            break;
        }

        case 7:
            cout << "Stop.\n";

            for (int i = 0; i < numar_apartamente; ++i)
            {
                delete apartamente[i];
            }

            return;

        default:
            cout << "Optiune invalida.\n";
            break;
        }
    }
}

int main() {
    bool b;

    do
    {
        meniu();
        std::cout << "Reia?";
        std::cin >> b;
    } while (b);

    return 0;
}
