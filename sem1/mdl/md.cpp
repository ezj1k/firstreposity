#include <iostream>
#include <cmath>
using namespace std;

// Funcțiile pentru operatorii logici

bool negare(bool A) {
    return !A;
}

bool conjuncție(bool A, bool B) {
    return A && B;
}

bool disjuncție(bool A, bool B) {
    return A || B;
}

bool implicație(bool A, bool B) {
    return !A || B;
}

bool echivalență(bool A, bool B) {
    return ((!A || B) && (!B || A));
}

// void tabel() {
// cout << "| A | B | C | F |" << endl << "| 0 | 0 | C | F |" << endl << "| 0 | 0 | C | F |" << endl << "| 0 | 1 | C | F |" << endl << "| 0 | 1 | C | F |" << endl << "| 1 | 0 | C | F |" << endl << "| 1 | 0 | C | F |" << endl << "| 1 | 1 | C | F |" << endl << "| 1 | 1 | C | F |" << endl;

// }

void tabel() {//    0 1 2 3 4   0 1 - 2^2 = 4  0 1 2 - 2^3 =8
    int n;
cin >> n;

int matrice[pow(2,n)][n];
for(int i=0;i<n-1;i++) {
for(int j =0;j<pow(2,n)/2; j++) {
matrice[j][i]=0;
}
for(int j=pow(2,n)/2;j<pow(2,n); j++) {
    matrice[j][i]=1;
    }
}
}

bool evaluează(const string& expresie, bool A_val, bool B_val, bool C_val) {
    bool A = A_val, B = B_val, C = C_val;
    
    bool rezultat = A;

    for (int i = 0; i < expresie.size(); ++i) {
        if (expresie[i] == '&') {
            if (i + 1 < expresie.size()) {
                if (expresie[i + 1] == 'A') {
                    rezultat = conjuncție(rezultat, A);
                } else if (expresie[i + 1] == 'B') {
                    rezultat = conjuncție(rezultat, B);
                } else if (expresie[i + 1] == 'C') {
                    rezultat = conjuncție(rezultat, C);
                }
            }
        }
        else if (expresie[i] == 'V') {
            if (i + 1 < expresie.size()) {
                if (expresie[i + 1] == 'A') {
                    rezultat = disjuncție(rezultat, A);
                } else if (expresie[i + 1] == 'B') {
                    rezultat = disjuncție(rezultat, B);
                } else if (expresie[i + 1] == 'C') {
                    rezultat = disjuncție(rezultat, C);
                }
            }
        }
        else if (expresie[i] == '->') {
            if (i + 1 < expresie.size()) {
                if (expresie[i + 1] == 'A') {
                    rezultat = implicație(rezultat, A);
                } else if (expresie[i + 1] == 'B') {
                    rezultat = implicație(rezultat, B);
                } else if (expresie[i + 1] == 'C') {
                    rezultat = implicație(rezultat, C);
                }
            }
        }
        else if (expresie[i] == '~') {
            if (i + 1 < expresie.size()) {
                if (expresie[i + 1] == 'A') {
                    rezultat = echivalență(rezultat, A);
                } else if (expresie[i + 1] == 'B') {
                    rezultat = echivalență(rezultat, B);
                } else if (expresie[i + 1] == 'C') {
                    rezultat = echivalență(rezultat, C);
                }
            }
        }
        else if (expresie[i] == '!') {
            if (i + 1 < expresie.size()) {
                if (expresie[i + 1] == 'A') {
                    rezultat = negare(A);
                } else if (expresie[i + 1] == 'B') {
                    rezultat = negare(B);
                } else if (expresie[i + 1] == 'C') {
                    rezultat = negare(C);
                }
            }
        }
    }
    return rezultat;
}

int main() {
    bool A, B, C;

    // Citește valorile de adevăr pentru variabile
    cout << "Introdu valoarea pentru A (0 sau 1): ";
    cin >> A;
    cout << "Introdu valoarea pentru B (0 sau 1): ";
    cin >> B;
    cout << "Introdu valoarea pentru C (0 sau 1): ";
    cin >> C;

    string expresie;
    cout << "Introduceti expresia logica (cu: &, V, ->, ~, !): ";
    cin >> expresie;


    return 0;
}
