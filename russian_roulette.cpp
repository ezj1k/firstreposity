#include <iostream>

using namespace std;

int main() {
    int fire, players, bullets;
    cout << "How much bullets are in pistol?" << endl;
    cin >> bullets;
    cout << "How much players join to table?" << endl;
    cin >> players;

while (players > 5 || players < 2) {
    cout << "Minimal 2 player, and maximal 5! Rewrite" << endl;
    cin >> players;
}

while (players>1) {
    fire = 1+rand()%bullets;
    cout << fire << endl;
    cout << "Game starts!" << endl;
    for(int i=1; i<=players; i++) {
        if (i%players==fire%players) {
            cout << "Player " << i << " is dead." << endl;
            players--;
        }
        else {
            cout << "Lucky! Player remain alive." << endl;
        }
    }
}
cout << "Game Over! 1 player remain and he wins";

    return 0;
}