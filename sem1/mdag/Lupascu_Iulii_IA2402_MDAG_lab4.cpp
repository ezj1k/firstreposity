#include <iostream>

using namespace std;

const int N = 100; // numărul maxim de vârfuri

int main() {
    int n, m; // numărul de vârfuri si muchii
    int adiacenta[N][N]; // matrice de adiacență
    int culoare[N]; // culoarea fiecărui vârf
    bool folosit[N]; // culori deja folosite de vecini

    cout << "Introdu numarul de varfuri: ";
    cin >> n;
    cout << "Introdu numarul de muchii: ";
    cin >> m;
    if (m > (n * (n - 1)) / 2) {
        cout << "Numar invalid de muchii!" << endl;
    } else {

    for (int i = 0; i < n; ++i) {
        culoare[i] = 0; // inițial toate vârfurile sunt necolorate
        for (int j = 0; j < n; ++j) {
            adiacenta[i][j]=0;
        }
    }

    int a, b;
    for (int i = 0; i < m; ++i) {
        do {
            cout << "Introdu varfurile care formeaza muchia #" << i + 1 << ": ";
            cin >> a >> b;
    
            if (a < 0 || b < 0 || a >= n || b >= n) {
                cout << "Indice invalid! Introdu varfuri între 0 și " << n - 1 << ".\n";
            } else if (adiacenta[a][b] != 0) {
                cout << "Muchia deja exista! Încearcă altă pereche.\n";
            } else {
                adiacenta[a][b] = adiacenta[b][a] = 1;
                cout << "S-a format muchia\n";
                break;
            }
        } while (true);
    }
    

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            cout << adiacenta[i][j];
        } //afiseaza matricea de adiacenta
        cout << endl;
    }
    // Colorare
    for (int v = 0; v < n; ++v) {
        for (int i = 0; i < n; ++i){ //se anuleaza la fiecare ciclu, caci se schimba vecinii
            folosit[i] = false;
        }
        // Marchează culorile vecinilor
        for (int u = 0; u < n; ++u) {
            if (adiacenta[v][u] == 1 && culoare[u] != 0){ //culoarea cu numarul dat se schimba pe folosita
                folosit[culoare[u]] = true;
            }
        }

        // Găsește prima culoare disponibilă
        for (int c = 1; c <= n; ++c) {
            if (!folosit[c]) { //se alege cea mai mica culoare disponibila, adica nefolosita de vecini.
                culoare[v] = c;
                break;
            }
        }
    }

    // === Afișare rezultate ===
    cout << "\nColorarea vârfurilor:\n";
    for (int i = 0; i < n; ++i) {
        cout << "Vârful " << i + 1 << " are culoarea " << culoare[i] << endl;
    }
    }
    return 0;
}
//a=0, b=1, c=2, d=3, e=4, f=5, g=6


