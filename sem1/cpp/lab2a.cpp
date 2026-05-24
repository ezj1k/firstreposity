#include <iostream>
#include <algorithm>

using namespace std;

class Pacient {
private:
    string diagnoza;
    int varsta;
protected:
    string numele, numele_medicului;
public:
    Pacient() { 
        numele = "";
        numele_medicului = "";
    }

    Pacient(string x, string y) {
        numele = x;
        numele_medicului = y;
    }

    virtual ~Pacient() {
        clog << "Numele pacientului: " << numele << "; Numele medicului: " << numele_medicului << ";" << endl;
    }

    void setNumele(string numele) { this->numele = numele; }
    void setNumeMedic(string numele_medicului) { this->numele_medicului = numele_medicului; }

    string getNumele() const { return numele; }
    string getNumeMedic() const { return numele_medicului; }

    virtual void citire() {
        cout << "Introduceti numele pacientului: " << endl;
        cin >> numele;
        cout << "Introduceti numele doctorului: " << endl;
        cin >> numele_medicului;
    }

    virtual void afisare() const {
        cout << "numele pcientului: " << numele << "; numele dr.: " << numele_medicului << "; " << endl;
    }
};

class Pprioritar : public Pacient {
private:
    int cost;
    string data_iternarii;
public:
    Pprioritar(): Pacient() {
        cost = 0;
        data_iternarii = "";
    }
    Pprioritar(string x, string y, int c, string d): Pacient(x,y) {
        cost = c;
        data_iternarii = d;
    }

    ~Pprioritar() {
        clog << "Cost: " << cost << "; Data iternarii: " << data_iternarii << ";\n";
    }

    void setCost(int cost) { this->cost = cost; }
    void setTipOp(string data_iternarii) { this->data_iternarii = data_iternarii; }

    int getCost() const { return cost; }
    string getTipOp() const { return data_iternarii; }

    void citire() override {
        Pacient::citire();
        cout << "Introduceti costul: " << endl;
        cin >> cost;
        cout << "Introduceti data iternarii: " << endl;
        cin >> data_iternarii;
    }

    void afisare() const override {
        Pacient::afisare();
        cout << "costul: " << cost << "; data iternarii: " << data_iternarii << "; " << endl;
    }
};

class Pobisnuit : public Pacient {
private:
    int cost;
    string data_iternarii;
public:
Pobisnuit() : Pacient() {
        cost = 0;
        data_iternarii = "";
    }
    Pobisnuit(string x, string y, int c, string d): Pacient(x,y) {
        cost = c;
        data_iternarii = d;
    }

    ~Pobisnuit() {
        clog << "Cost (pacient obisnuit): " << cost << "; data iternarii (pacient obisnuit): " << data_iternarii << ";\n";
    }

    void setCost(int cost) { this->cost = cost; }
    void setTipOp(string data_iternarii) { this->data_iternarii = data_iternarii; }

    int getCost() const { return cost; }
    string getTipOp() const { return data_iternarii; }

    void citire() override {
        Pacient::citire();
        cout << "Introduceti costul: " << endl;
        cin >> cost;
        cout << "Introduceti data iternarii: " << endl;
        cin >> data_iternarii;
    }

    void afisare() const override {
        Pacient::afisare();
        cout << "costul: " << cost << "; data iternarii: " << data_iternarii << "; " << endl;
    }
};

void meniu() {
    int maxPacienti = 100;
    Pacient* pacienti[maxPacienti];  // Tablou de pointeri la Pacient
    int indexPacient = 0;

    for (;;) {
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
    
                if (n < 0 || n > maxPacienti) {
                    cout << "Numar invalid de pacienti. Puteti adauga maxim " << maxPacienti << " pacienti.\n";
                    n = maxPacienti;
                }
    
                if (indexPacient + n > maxPacienti) {
                    cout << "Nu mai sunt locuri disponibile pentru alți pacienți.\n";
                    n = maxPacienti - indexPacient;
                }
            int alegere;
            while (alegere!=1 || alegere!=2) {
            cout << "Este pacientul 1)Obisnuit; sau 2)Prioritar" << endl;
            cin >> alegere;
            if (alegere==2) {
                for (int i = 0; i < n; i++) {
                    pacienti[indexPacient] = new Pprioritar;
                    pacienti[indexPacient]->citire();
                    indexPacient++;
                }
            }
            else if (alegere==1) {
                for (int i = 0; i < n; i++) {
                    pacienti[indexPacient] = new Pobisnuit;
                    pacienti[indexPacient]->citire();
                    indexPacient++;
                }
            }
            else {
                cout << "Introdu o valoare corecta" << endl;
            }
            }
                break;
            }

        case 2: {
            cout << "\nAfisare pacienți:\n";
            for (int i = 0; i < indexPacient; i++) {
                pacienti[i]->afisare();
                cout << endl;
            }
            break;
        }

        case 3: {
//Dupa nume
            sort(pacienti, pacienti + indexPacient, [](Pacient* a, Pacient* b) {
                return a->getNumele() < b->getNumele();
            });

            cout << "\nPacienți sortați:\n";
            for (int i = 0; i < indexPacient; i++) {
                pacienti[i]->afisare();
                cout << endl;
            }
            break;
        }

        case 4: {
            int bugetobisnuiti, bugetprioritari;
            cout << "Introduceti bugetul de care dispun pacientii obisnuiti si apoi cei prioritari: ";
            cin >> bugetobisnuiti >> bugetprioritari;
        
            for (int i = 0; i < indexPacient; i++) {
                pacienti[i]->afisare(); // Afișăm informațiile pacientului
        
                // Pentru pacientii de tip 'xxx', verificăm costul
                if (Pprioritar* pacientX = dynamic_cast<Pprioritar*>(pacienti[i])) {
                    if (bugetprioritari >= pacientX->getCost()) {
                        cout << "Pacientul prioritar are bani de operatie" << endl;
                    }
                    else {
                        cout << "Pacientul prioritar nu are bani de operatie" << endl;
                    }
                }
        
                // Pentru pacientii de tip 'yyy', verificăm varsta
                else if (Pobisnuit* pacientY = dynamic_cast<Pobisnuit*>(pacienti[i])) {
                    if (bugetobisnuiti <= pacientY->getCost()) {
                        cout << "Pacientul obisnuit are bani de operatie" << endl;
                    }
                    else {
                        cout << "Pacientul obisnuit nu are bani de operatie" << endl;
                    }
                }
                cout << "-------------" << endl;
            }
            break;
        }

        case 5: {
            int k;
            cout << "Introduceti pozitia (k) pentru schimbarea acestui obiect: ";
            cin >> k;
        
            // Verificăm dacă poziția este validă
            if (k >= 0 && k < indexPacient) {
                pacienti[k]->citire(); // Citim datele pacientului la pozitia k
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
                // Verificăm pentru pacientii prioritari
                if (Pprioritar* pacientX = dynamic_cast<Pprioritar*>(pacienti[i])) {
                    if (pacientX->getTipOp() == p) {
                        delete pacienti[i];
                        // Mutăm elementele pentru a "șterge" pacientul
                        for (int j = i; j < indexPacient - 1; ++j) {
                            pacienti[j] = pacienti[j + 1];
                        }
                        indexPacient--;
                        gasit = true;
                        break;  // Ieșim din buclă
                    }
                }
        
                // Verificăm pentru pacientii de tip 'yyy'
                else if (Pobisnuit* pacientY = dynamic_cast<Pobisnuit*>(pacienti[i])) {
                    if (pacientY->getTipOp() == p) {
                        delete pacienti[i];
                        // Mutăm elementele pentru a "șterge" pacientul
                        for (int j = i; j < indexPacient - 1; ++j) {
                            pacienti[j] = pacienti[j + 1];
                        }
                        indexPacient--;
                        gasit = true;
                        break;  // Ieșim din buclă
                    }
                }
            }
        
            if (!gasit) {
                cout << "Nu s-a gasit obiectul cu valoarea specificata.\n";
            }
            break;
        }

        case 7:
            cout << "Stop.\n";

            for (int i = 0; i < indexPacient; ++i)
            {
                delete pacienti[i];
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
