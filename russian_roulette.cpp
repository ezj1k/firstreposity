#include <iostream>
#include <cstdlib>
#include <ctime>

using namespace std;

int main() {
    srand(time(NULL));
    int fire, players, bullets, dead[5]{}, j=0, winner;
    cout << "How much players join to table?" << endl;
    cin >> players;

while (players > 5 || players < 2) {
    cout << "Minimal 2 player, and maximal 5! Rewrite" << endl;
    cin >> players;
}

cout << "Revolver has 7 slots" << endl;
while(players-j>1){
    for(int i=1; i<=players; ++i) {
        if(j==players-1){
            break;
        }
        bool isDead = false;
        for(int k = 0; k < j; ++k) {
            if(i == dead[k]){
                isDead = true;
                break;
            }
        }
        if(isDead){continue;}

        cout << "How much bullets are in pistol?" << endl;
        cin >> bullets;
        while (bullets > 6 || bullets < 1) {
            cout << "Minimal 1 bullet, and maximal 6! Rewrite" << endl;
            cin >> bullets;
        }
        fire=1+rand()%7;
        if(7-bullets>=fire){
            cout << "Player " << i << " didn't die\n";
        }
        else{
            cout << "Player " << i << " died\n";
            dead[j]=i;
            ++j;
        }
    }
    for (int i = 1; i <= players; ++i) {
        bool isDead = false;
        for (int k = 0; k < j; ++k) {
            if (i == dead[k]) {
                isDead = true;
                break;
            }
        }
        if (!isDead) {
            cout << "Player " << i << " is still alive\n";
        }
    }
}

for (int i = 1; i <= players; ++i) {
    bool isDead = false;
    for (int k = 0; k < j; ++k) {
        if (dead[k] == i) {
            isDead = true;
            break;
        }
    }
    if (!isDead) {
        winner = i;
        break;
    }
}

cout << "Player " << winner << " wins the game!";

    return 0;
}
