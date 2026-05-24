#include <iostream>
using namespace std;

class Graf {
public:
int nrnoduri;
int matrice[50][50];
bool vizitat[50];

Graf(int n) {
    nrnoduri=n;

    for(int i=0;i<nrnoduri;i++) {
        for(int j=0; j<nrnoduri; j++) {
            matrice[i][j]=0;
        }
    }
}

void adaugaMuchie(int v, int u) {
    matrice[u][v]=1;
    matrice[v][u]=1;
}

void dfs(int first) {
    for(int i=0;i<nrnoduri;i++) {
        vizitat[i]=false;
    }
    dfsfunc(first);
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

private:
void dfsfunc(int nod) {
    vizitat[nod]=true;
    cout << nod << " ";
    for(int i=0;i<nrnoduri; i++) {
        if(matrice[nod][i]==1 && !vizitat[i]) {
            dfsfunc(i);   
        }
    }
}
};

int main() {
    int n, a, b, m;
    cout << "Introdu numarul de noduri: " << endl;
    cin >> n;
    cout << "Introdu numarul de muchiii(mai mic decat combinari n a cate 2): " << endl;

    cin >> m;
    if(m>(n*(n-1))/2) {
    cout << "Ai depasit valoarea corecta" << endl;   
    }
    else {
    Graf Graf(n);
    for(int i=0; i<m;i++) {
        cout << "Pentru " << i+1 << "-a muchie: ";
        cin >> a >> b;
        Graf.adaugaMuchie(a, b);
    }

    Graf.afiseazaMatricea();

    cout << "raspuns: ";
    Graf.dfs(0);
    }
    
    return 0;
}
