#include <iostream>
#include <string>
#include <cstdlib>
#include <ctime>

using namespace std;
//deschide(xcor, ycor, spacex, spacey, space, space2);
void deschide(int* x, int* y, int* spacex, int* spacey, int** space, string** space2) {
    if (*x < 0 || *x >= *spacex || *y < 0 || *y >= *spacey)
        return;
    if (space2[*x][*y] != "x" || space[*x][*y] == 9)
        return;

    space2[*x][*y] = to_string(space[*x][*y]);

    if (space[*x][*y] != 0)
        return;

    for (int dx = -1; dx <= 1; dx++) {
        for (int dy = -1; dy <= 1; dy++) {
            if (dx != 0 || dy != 0) {
                int nx = *x + dx;
                int ny = *y + dy;
                deschide(&nx, &ny, spacex, spacey, space, space2);
            }
        }
    }
}



int main() {
    srand(time(NULL));
    int spacex, spacey;
    cout << "Introduce the area (max: 20x20; min: 2x2)" << endl;
    cin >> spacex >> spacey;

    while(spacex>20 || spacex<2 || spacey>20 || spacey<2){
        cout << "Incorect number, introduce a new\n";
        cin >> spacex >> spacey;
    }

    unsigned int bombs, x, y;
    cout << "Introduce a number of bombs(less than a slots of area):\n";
    cin >> bombs;
    
    while(bombs>spacex*spacey || bombs<=0){
        if(bombs==0){
            cout << "Cant be 0 bombs! Introduce a new\n";
            cin >> bombs;
        }
        else {
            cout << "Cant be to much bombs! Introduce a new\n";
            cin >> bombs;
        }
    }

    int **space = new int*[spacex];
for (int i = 0; i < spacex; i++) {
    space[i] = new int[spacey];
    for (int j = 0; j < spacey; j++) {
        space[i][j] = 0;
    }
}

    int placed = 0;
    while (placed < bombs) {
        x = rand() % spacex;
        y = rand() % spacey;
        if (space[x][y] != 9) {
            space[x][y] = 9;
            ++placed;
        }
    }

    int ni, nj;
    for(int i=0; i<spacex; ++i){
        for(int j=0; j<spacey; ++j){
            if(space[i][j]==9){
                for(int dx=-1; dx<=1; ++dx){
                    for(int dy=-1; dy<=1; ++dy){
                        ni = i+dx;
                        nj = j+dy;
                        if(dx==0 && dy==0){
                            continue;
                        }
<<<<<<< HEAD
                        if(ni>=0 && ni<spacex && nj>=0 && nj<spacey && space[ni][nj]!=9){
=======
                        if(ni>=0 && ni<spacex && nj>=0 && nj<spacey && space[ni][nj]){
>>>>>>> 69a75e0943bbe66c32f3d2cb739ab1143ae62cd8
                            ++space[ni][nj];
                        }
                    }
                }
            }
        }
    }
//------------------------------------------------
    string **space2 = new string*[spacex];
    for (int i = 0; i < spacex; i++) {
        space2[i] = new string[spacey];
        for (int j = 0; j < spacey; j++) {
            space2[i][j] = "x";
        }
    }

    for(int i=0; i<spacex; ++i){
        for(int j=0; j<spacey; ++j){
            cout << space2[i][j];
        }
        cout << endl;
    }

    int xcor=-1, ycor=-1;
    cout << "Introduce coordonates of slot you choose(within of area):\n";
    cin >> xcor >> ycor;
    --xcor;
    --ycor;

    while (xcor < 0 || xcor >= spacex || ycor < 0 || ycor >= spacey) {
        cout << "Invalide coordonates. Rewrite them:\n";
        cin >> xcor >> ycor;
        --xcor;
        --ycor;
    }

    while(space[xcor][ycor]!=9){
        int xes=0;
        while (xcor < 0 || xcor >= spacex || ycor < 0 || ycor >= spacey) {
            cout << "Invalide coordonates. Rewrite them:\n";
            cin >> xcor >> ycor;
            --xcor;
            --ycor;
        }
        while(space2[xcor][ycor]!="x"){
            cout << "Is selected, change your slot\n";
            cin >> xcor >> ycor;
            --xcor;
            --ycor;
        }
        
        deschide(&xcor, &ycor, &spacex, &spacey, space, space2);
        for (int i = 0; i < spacex; ++i) {
            for (int j = 0; j < spacey; ++j) {
                if(space2[i][j]=="x"){
                    ++xes;
                }
            }
        }

        for (int i = 0; i < spacex; ++i) {
            for (int j = 0; j < spacey; ++j) {
                cout << space2[i][j];
            }
            cout << endl;
        }

        if(xes!=bombs){
            cout << "Introduce next slot\n";
            cin >> xcor >> ycor;
        --xcor;
        --ycor;
        }
        else{
            cout << "You win!\n";
            break;
        }
    }
    for (int i = 0; i < spacex; i++) {
        delete[] space[i];
        delete[] space2[i];
    }
    delete[] space;
    delete[] space2;
    cout << "Game over!";


    return 0;
}
