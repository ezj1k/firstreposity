#include <iostream>
#include <vector>
using namespace std;

class Graf {
public:
    int nrnoduri;
    int matrice[50][50];

    Graf(int n) {
        nrnoduri = n;
        for (int i = 0; i < nrnoduri; i++) {
            for (int j = 0; j < nrnoduri; j++) {
                matrice[i][j] = 0;
            }
        }
    }

    void adaugaMuchie(int v, int u) {
        matrice[u][v] = 1;
        matrice[v][u] = 1;
    }

    void afiseazaMatricea() {
        cout << "Matricea de adiacenta este: \n";
        for (int i = 0; i < nrnoduri; i++) {
            for (int j = 0; j < nrnoduri; j++) {
                cout << matrice[i][j] << " ";
            }
            cout << endl;
        }
    }

    void gasesteS() {
        bool exclus[50] = {false};  // Marcam nodurile excluse
        int k = 0;  // Contor pentru multimea S
        int S[50];  // Multimea finala a nodurilor excluse

        while (k < nrnoduri) {
            int gradMin = nrnoduri + 1;
            int varfMin = -1;

            // Calculam gradele fiecarui varf (luand in considerare doar vecinii nevizitati)
            for (int i = 0; i < nrnoduri; i++) {
                if (exclus[i] == false) {  // Verificam doar nodurile neexcluse
                    int grad = 0;
                    for (int j = 0; j < nrnoduri; j++) {
                        if (matrice[i][j] == 1 && exclus[j] == false) {
                            grad++;  // Contează vecinii nevizitați
                        }
                    }
                    if (grad < gradMin) {
                        gradMin = grad;
                        varfMin = i;
                    }
                }
            }

            if (varfMin == -1) {
                // Nu mai există noduri cu vecini nevizitați, deci terminăm
                break;
            }

            // Adaugam varful cu gradul minim in multimea S
            S[k++] = varfMin;
            exclus[varfMin] = true;  // Marcam varful ca fiind exclus
            cout << "Varful cu gradul minim: " << varfMin << endl;

            // Excludem vecinii acestui varf
            for (int i = 0; i < nrnoduri; i++) {
                if (matrice[varfMin][i] == 1 && exclus[i] == false) {
                    exclus[i] = true;  // Excludem vecinul
                    cout << "Vecin exclus: " << i << endl;
                }
            }
        }

        // Afisam multimea S (noduri excluse)
        cout << "Multimea S: ";
        for (int i = 0; i < k; i++) {
            cout << S[i] << " ";
        }
        cout << endl;
    }
};

int main() {
    int n, a, b, m;
    cout << "Introdu numarul de noduri: " << endl;
    cin >> n;
    cout << "Introdu numarul de muchii (mai mic decat combinari n a cate 2): " << endl;
    cin >> m;

    if (m > (n * (n - 1)) / 2) {
        cout << "Ai depasit valoarea corecta" << endl;
    } else {
        Graf Graf(n);
        for (int i = 0; i < m; i++) {
            cout << "Pentru " << i + 1 << "-a muchie: ";
            cin >> a >> b;
            Graf.adaugaMuchie(a, b);
        }

        Graf.afiseazaMatricea();

        // Calculam si afisam multimea S
        Graf.gasesteS();
    }

    return 0;
}
