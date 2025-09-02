#include <iostream>
#include <climits>
using namespace std;

class Graf {
public:
    int nrnoduri;
    int matrice[50][50];

    // Constructorul clasei
    Graf(int n) {
        nrnoduri = n;
        // Inițializăm matricea de adiacență
        for (int i = 0; i < nrnoduri; i++) {
            for (int j = 0; j < nrnoduri; j++) {
                matrice[i][j] = 0;
            }
        }
    }

    // Funcția pentru adăugarea unei muchii
    void adaugaMuchie(int v, int u, int x) {
        matrice[u][v] = x;
        matrice[v][u] = x;
    }

    // Afișează matricea de adiacență
    void afiseazaMatricea() {
        cout << "Matricea de adiacenta este: \n";
        for (int i = 0; i < nrnoduri; i++) {
            for (int j = 0; j < nrnoduri; j++) {
                cout << matrice[i][j] << " ";
            }
            cout << endl;
        }
    }

    // Algoritmul lui Prim pentru a găsi arborele de acoperire minimă
    void prim() {
        bool inMST[50] = {false};  // Marcam nodurile deja incluse în MST
        int costtotal = 0;        // Ponderea totală a MST-ului
        int mstEdges[50][3];        // Muchiile din MST (u, v, weight)
        int mstEdgeCount = 0;       // Contor pentru muchiile din MST

        // Începem de la nodul 0
        inMST[0] = true;

        // Iterăm până când avem n-1 muchii în MST
        while (mstEdgeCount < nrnoduri - 1) {
            int costmin = INT_MAX;  // Ponderea minimă
            int u = -1, v = -1;       // Vârfurile care formează muchia cu pondere minimă

            // Căutăm muchia cu pondere minimă care conectează un vârf din MST cu unul nou
            for (int i = 0; i < nrnoduri; i++) {
                if (inMST[i]) {
                    for (int j = 0; j < nrnoduri; j++) {
                        if (!inMST[j] && matrice[i][j] != 0 && matrice[i][j] < costmin) {
                            costmin = matrice[i][j];
                            u = i;
                            v = j;
                        }
                    }
                }
            }

            // Adăugăm muchia la MST
            mstEdges[mstEdgeCount][0] = u;
            mstEdges[mstEdgeCount][1] = v;
            mstEdges[mstEdgeCount][2] = costmin;
            mstEdgeCount++;
            costtotal += costmin;

            // Marcam vârfurile conectate de această muchie
            inMST[u] = inMST[v] = true;
        }

        // Afișăm muchiile din MST și ponderea totală
        cout << "Muchiile din arborele de acoperire minimă sunt:" << endl;
        for (int i = 0; i < mstEdgeCount; i++) {
            cout << "(" << mstEdges[i][0] + 1 << ", " << mstEdges[i][1] + 1 << ") = " << mstEdges[i][2] << endl;
        }
        cout << "Ponderea totală a MST-ului: " << costtotal << endl;
    }
};

int main() {
    int n, a, b, x, m;
    
    cout << "Introdu numarul de noduri: " << endl;
    cin >> n;

    // Creăm un obiect Graf cu n noduri
    Graf graf(n);

    cout << "Introdu numarul de muchii: " << endl;
    cin >> m;

    // Verificăm să nu depășim numărul maxim de muchii pentru un graf complet
    if (m > (n * (n - 1)) / 2) {
        cout << "Numar invalid de muchii!" << endl;
    } else {
        // Citim muchiile și ponderile lor
        for (int i = 0; i < m; i++) {
            cout << "Pentru muchia " << i + 1 << ", introduceti varful de start, varful de final si ponderea: ";
            cin >> a >> b >> x;
            graf.adaugaMuchie(a - 1, b - 1, x);  // Decrementam pentru a face indecsii corecti in matricea de adiacenta
        }

        // Afișăm matricea de adiacență
        graf.afiseazaMatricea();

        // Aplicăm algoritmul lui Prim pentru a găsi arborele de acoperire minimă
        graf.prim();
    }

    return 0;
}

//a=1, b=2, c=3, d=4, e=5, f=6, g=7