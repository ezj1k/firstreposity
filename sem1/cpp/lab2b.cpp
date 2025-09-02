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

    virtual int getCost() const = 0;
    virtual string getTipOp() const = 0;

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

class Pprioritar : public virtual Pacient {
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

    ~Pprioritar() override {
        clog << "Cost: " << cost << "; Data iternarii: " << data_iternarii << ";\n";
    }

    void setCost(int cost) { this->cost = cost; }
    void setTipOp(string data_iternarii) { this->data_iternarii = data_iternarii; }

    int getCost() const override { return cost; }
    virtual string getTipOp() const override { return data_iternarii; }

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

    virtual int CostX() const {return cost;}
};

class Pobisnuit : public virtual Pacient {
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

    ~Pobisnuit() override {
        clog << "Cost (pacient obisnuit): " << cost << "; data iternarii (pacient obisnuit): " << data_iternarii << ";\n";
    }

    void setCost(int cost) { this->cost = cost; }
    void setTipOp(string data_iternarii) { this->data_iternarii = data_iternarii; }

    int getCost() const override { return cost; }
    virtual string getTipOp() const override { return data_iternarii; }

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

    virtual string DataIternariiY() const {return data_iternarii;}
};

class Copil : public Pprioritar, public Pobisnuit {
private:
bool bolnav;
public:
    Copil() {
        bolnav="";
    }

    Copil(bool z) {
        bolnav=z;
    }

    ~Copil() override {
        clog << "Copilul este bolnav " << bolnav << endl;
    }

    void setBolnav(bool copilas) {this->bolnav=copilas;}
    bool getBolnav() const {return bolnav;}

    int getCost() const override {return Pprioritar::getCost();}
    string getTipOp() const override {return Pprioritar::getTipOp();}

    void citire() override {
        Pobisnuit::citire();
    }

    void citire(bool estePrioritar) {
        if (estePrioritar) {
            Pprioritar::citire();
        }
        else {
            citire();
        }
    }

    void afisare() const override {
        Pobisnuit::afisare();
    }

    void afisare(bool estePrioritar) const {
        if (estePrioritar) {
            Pprioritar::afisare();
        }
        else {
            afisare();
        }
    }
};

void meniu() {
    int maxPacienti = 100;
    Pacient* pacienti[maxPacienti];  // Tablou de pointeri la Pacient
    int indexPacient = 0;

    for (;;) {
        cout << "\nMeniu:\n";
        cout << "1. citirea de la tastatură a valorilor membrilor obiectelor diferitor clase din ierarhie în tabloul definit;\n";
        cout << "2. afișarea obiectelor tabloului la ecran;\n";
        cout << "3. sortarea obiectelor în ordine definită de student după un câmp comun tuturor claselor din ierarhie;\n";
        cout << "4. afișarea la ecran a obiectelor pentru care se cunoaște că un câmp comun tuturor claselor din ierarhie satisface o conditie formulată de student;\n";
        cout << "5. se modifică datele obiectului pe poziția k în tabloul de obiecte (k se citește de la tastatură);\n";
        cout << "6. se șterge obiectul pentru care se cunoaște că un câmp comun tuturor claselor din ierarhiit::afisare();e satisface o condiție formulată de student;\n";
        cout << "7. Stop;\n";
        cout << "Alegeti optiunea: ";

        int optiune;
        cin >> optiune;

        switch (optiune) {
            case 1: {
                int n;
                cout << "Introduceti numarul de pacienti (maxim " << maxPacienti << "): ";
                cin >> n;
    
                while (n < 0 || n > maxPacienti) {
                    cout << "Numar invalid de pacienti. Puteti adauga maxim " << maxPacienti << " pacienti.\n";
                    cin >> n;
                }
    
                if (indexPacient + n > maxPacienti) {
                    cout << "Nu mai sunt locuri disponibile pentru alți pacienți.\n";
                    n = maxPacienti - indexPacient;
                }
            int alegere;
            do {
            cout << "Este pacientul 1)Obisnuit; sau 2)Prioritar; sau 3)Copil;" << endl;
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
            else if (alegere==3) {
                for (int i = 0; i < n; i++) {
                    Copil *copil = new Copil;
                    pacienti[indexPacient] = copil;

                    bool bolnav;
                    cout << "Copilul dat este pacient prioritar?(da/nu): " << endl;
                    cin >> bolnav;

                    copil->citire(bolnav);
                    indexPacient++;
                }
            }
            else {
                cout << "Introdu o valoare corecta" << endl;
            }
            } while (alegere!=1 && alegere!=2 && alegere!=3);
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
            float bugetobisnuiti, bugetprioritari, bugetparinti;
            cout << "Introduceti bugetul de care dispun pacientii obisnuiti si apoi cei prioritari si parintii copiilor: ";
            cin >> bugetobisnuiti >> bugetprioritari >> bugetparinti;
            
            
            for (int i = 0; i < indexPacient; i++) {
                pacienti[i]->afisare(); // Afișăm informațiile pacientului
                
                // Pentru pacientii de tip 'prioritari', verificăm costul
                if (dynamic_cast<Pprioritar*>(pacienti[i])!=NULL) {
                    if (bugetprioritari >= pacienti[i]->getCost()) {
                        cout << "Pacientul prioritar are bani de operatie" << endl;
                    }
                    else {
                        cout << "Pacientul prioritar nu are bani de operatie" << endl;
                    }
                }
                
                // Pentru pacientii de tip 'obisnuit'
                else if (dynamic_cast<Pobisnuit*>(pacienti[i])!=NULL) {
                    if (bugetobisnuiti >= pacienti[i]->getCost()) {
                        cout << "Pacientul obisnuit are bani de operatie" << endl;
                    }
                    else {
                        cout << "Pacientul obisnuit nu are bani de operatie" << endl;
                    }
                }

                else if (dynamic_cast<Copil*>(pacienti[i]) !=NULL) {
                    if (bugetparinti >= pacienti[i]->getCost()) {
                        cout << "Pacientul este copil si parintii sai au bani de operatie" << endl;
                    }
                    else {
                        cout << "Pacientul este copil, dar parintii sai nu au bani de operatie" << endl;
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
                    if (pacienti[i]->getNumele() == p) {
                        delete pacienti[i];
                        // Mutăm elementele pentru a "șterge" pacientul
                        for (int j = i; j < indexPacient - 1; ++j) {
                            pacienti[j] = pacienti[j + 1];
                        }
                        pacienti[indexPacient-1] = nullptr;
                        indexPacient--;
                        gasit = true;
                        break;  // Ieșim din buclă
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
