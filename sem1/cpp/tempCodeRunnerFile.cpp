#include <iostream>
#include <algorithm>

using namespace std;

class Pacient {
private:
string data_internării, diagnoza;
protected:
string numele, numele_medicului;
public:
Pacient() { 
    numele="";
    numele_medicului="";
}

Pacient(string x, string y) {
numele=x;
numele_medicului=y;
}

virtual ~Pacient() { // de intrebat virtual
clog << "Numele: " << numele << "; Numele medicului: " << numele_medicului << ";" << endl;
}

void setNumele(string numele) { this->numele = numele; }
void setNumeMedic(string numele_medicului) { this->numele_medicului = numele_medicului; }

string getNumele() const { return numele; }
string getNumeMedic() const { return numele_medicului; }

void citire() {
cout << "Introduceti numele pacientului: " << endl;
cin >> numele;
cout << "Introduceti numele doctorului: " << endl;
cin >> numele_medicului;
}

void afisare() {
cout << "numele: " << numele << "; numele dr.: " << numele_medicului << "; " << endl;
}
};

class xxx: public Pacient {
private:
int cost;
string tip_operatie;
public:
xxx(): Pacient() {
cost=0;
tip_operatie="";
}
xxx(string x, string y, int c, string t): Pacient(x,y) {
cost=c;
tip_operatie=t;
}

~xxx() {
clog << "Cost: " << cost << "; Tip de operatie: " << tip_operatie << ";\n";
}

void setCost(int cost) { this->cost = cost; }
void setTipOp(string tip_operatie) { this->tip_operatie = tip_operatie; }

int getCost() const { return cost; }
string getTipOp() const { return tip_operatie; }

void aciti() {
    citire();
    cout << "Introduceti costului: " << endl;
    cin >> cost;
    cout << "Introduceti tipul de operatie: " << endl;
    cin >> tip_operatie;
}

void aafisa() {
    afisare();
    cout << "costul: " << cost << "; tipul de operatie: " << tip_operatie << "; " << endl;
    }
};

class yyy: public Pacient {
    private:
    string cabinet;
    int varsta;
    public:
    yyy(): Pacient() {
    cabinet="";
    varsta=0;
    }
    yyy(string x, string y, string c, int t): Pacient(x,y) {
    cabinet=c;
    varsta=t;
    }
    
    ~yyy() {
        clog << "Cabinet: " << cabinet << "; Varsta: " << varsta << ";\n";
        }

    void setCabinet(string cabinet) { this->cabinet = cabinet; }
    void setVarsta(int varsta) { this->varsta = varsta; }
    
    string getCabinet() const { return cabinet; }
    int getVarsta() const { return varsta; }

    void aciti() {
        citire();
        cout << "Introduceti cabinetul: " << endl;
        cin >> cabinet;
        cout << "Introduceti varsta: " << endl;
        cin >> varsta;
    }
    
    void aafisa() {
        afisare();
        cout << "cabinetul: " << cabinet << "; varsta: " << varsta << "; " << endl;
        }
    };

    bool compare_xxx(const xxx* a, const xxx* b) {
        return a->getTipOp() < b->getTipOp();
    }
    
    bool compare_yyy(const yyy* a, const yyy* b) {
        return a->getCabinet() < b->getCabinet();
    }

void meniu() {
    int maxPacienti=100;
    xxx* pacientiX[maxPacienti];
    yyy* pacientiY[maxPacienti];
    int indexPacient=0;

    for (; ; ) {
        cout << "\nMeniu:\n";
        cout << "1. citirea de la tastatură a valorilor membrilor obiectelor tabloului definit;\n";
        cout << "2. afișarea obiectelor tabloului la ecran;\n";
        cout << "3. sortarea obiectelor în ordine definită de student;\n";
        cout << "4. afișarea la ecran a obiectelor pentru care se cunoaște că un câmp satisface o condiție formulată de student;\n";
        cout << "5. se modifică date obiectului pe poziția k în tabloul de obiecte (k se citește de la tastatură);\n";
        cout << "6. se șterge obiectul pentru care se cunoaște că un câmp satisface o condiție formulată de student;\n";
        cout << "7. Stop;\n";
        cout << "Alegeti optiunea: ";

        int optiune;
        cin >> optiune;

        switch (optiune) {
        case 1: {
            int n;
            cout << "Introduceti numarul de pacienti (maxim " << maxPacienti << "): ";
            cin >> n;

            // Verificăm că nu depășim dimensiunea maximă a tabloului
            if (n < 0 || n > maxPacienti) {
                cout << "Numar invalid de pacienti. Puteti adauga maxim " << maxPacienti << " pacienti.\n";
                n = maxPacienti;
            }

            for (int i = 0; i < n; i++) {
                pacientiX[indexPacient] = new xxx;
                pacientiX[indexPacient]->aciti();
                indexPacient++;
            }
            for (int i = 0; i < n; i++) {
                pacientiY[indexPacient] = new yyy;
                pacientiY[indexPacient]->aciti();
                indexPacient++;
            }
            break;
        }

        case 2:
            for (int i = 0; i < indexPacient; i++) {
                pacientiX[i]->aafisa();
                cout << endl;
            }
            cout << "-----------------" << endl;
            for (int i = 0; i < indexPacient; i++) {
                pacientiY[i]->aafisa();
                cout << endl;
            }

            break;

            case 3: {
                // Sortăm pacienții de tip xxx
                sort(pacientiX, pacientiX + indexPacient, compare_xxx);
                
                // Afișăm pacienții după sortare
                cout << "Tipurile de operatii de tip xxx, sortati alfabetic:\n";
                for (int i = 0; i < indexPacient; i++) {
                    pacientiX[i]->aafisa();
                    cout << endl;
                }
                
                // Sortăm pacienții de tip yyy
                sort(pacientiY, pacientiY + indexPacient, compare_yyy);
                
                // Afișăm pacienții după sortare
                cout << "Cabinet de tip yyy, sortati alfabetic:\n";
                for (int i = 0; i < indexPacient; i++) {
                    pacientiY[i]->aafisa();
                    cout << endl;
                }
                
                break;
            }

        case 4: {
            int z,x;
            cout << "Introduceti bugetul (z) si varsta minima admisibila: ";
            cin >> z >> x;
            for (int i = 0; i < indexPacient; i++) {
                if (z <= pacientiX[i]->getCost()) {
                    pacientiX[i]->aafisa();
                    cout << "are bani de operatie" << endl;
                    cout << "-------------" << endl;
                }
            }
            for (int i = 0; i < indexPacient; i++) {
                if (z <= pacientiY[i]->getVarsta()) {
                    pacientiY[i]->aafisa();
                    cout << "are ani destuli" << endl;
                    cout << "-------------" << endl;
                }
            }
            break;
        }

        case 5: {
            int k;
            cout << "Introduceti pozitia (k) pentru schimbarea acestui obiect: ";
            cin >> k;

            // Verificăm dacă poziția este validă
            if (k >= 0 && k <= indexPacient) {
                    pacientiX[k]->aciti();
            } else {
                cout << "Pozitie invalida!\n";
            }

            cout << "Introduceti pozitia (k) pentru schimbarea acestui obiect: ";
            cin >> k;
            if (k >= 0 && k <= indexPacient) {
                pacientiY[k]->aciti();
        } else {
            cout << "Pozitie invalida!\n";
        }
            break;
        }

        case 6: {
            string p;
            cout << "Introduceti numele (p) pentru care doriti sa stergeti obiectul: ";
            cin >> p;

            bool gasit = false;
            for (int i = 0; i < indexPacient; ++i) {
                if (pacientiX[i]->getTipOp() == p) {
                    delete pacientiX[i];
                    // Mutăm elementele pentru a "șterge" apartamentul
                    for (int j = i; j < indexPacient - 1; ++j) {
                        pacientiX[j] = pacientiX[j + 1];
                    }
                    indexPacient--;
                    gasit = true;
                    break; //de intrebat
                }
            }
            if (!gasit) {
                cout << "Nu s-a gasit obiectul cu suprafata specificata.\n";
            }

            cout << "Introduceti numarul cabinetului (p) pentru care doriti sa stergeti obiectul: ";
            cin >> p;
            gasit = false;
            for (int i = 0; i < indexPacient; ++i) {
                if (pacientiY[i]->getCabinet() == p) {
                    delete pacientiY[i];
                    // Mutăm elementele pentru a "șterge" apartamentul
                    for (int j = i; j < indexPacient - 1; ++j) {
                        pacientiY[j] = pacientiY[j + 1];
                    }
                    indexPacient--;
                    gasit = true;
                    break; //de intrebat
                }
            }
            if (!gasit) {
                cout << "Nu s-a gasit obiectul cu suprafata specificata.\n";
            }
            break;
        }

        case 7:
            cout << "Stop.\n";

            for (int i = 0; i < indexPacient; ++i)
            {
                delete pacientiX[i];
                delete pacientiY[i];
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
        cout << "Reia?";
        cin >> b;
    } while (b);
    
    return 0;
}
