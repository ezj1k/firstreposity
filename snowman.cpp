#include <iostream>

using namespace std;

class Snowman {
private:
    int health;
public:
Snowman(int health) {
        this->health=health;
    }
    friend int func(Snowman &snowman);
};

class Temperature {
public:
void interact(Snowman &snowman) {
    func(snowman);
}};

int func(Snowman &snowman) {
    string op;
    int x=1;
    do {
        cout << "enter the temperature  (hot, cold, summer): " << endl;
    cin >> op;
    if (op=="summer") {
        snowman.health =0;
        cout << "The snowman melted and only the carrot remained." << endl;
        x=0;
    }
    else if (op=="hot") {
        snowman.health -=20;
        if (snowman.health<=0) {
            snowman.health =0;
            cout << "The snowman melted and only the carrot remained." << endl;
            x=0;
        }
        else {
        cout << "Snowman melts and has " << snowman.health << " hp." << endl;
        }
    }
    else if (op=="cold") {
        snowman.health +=10;
        if(snowman.health>100) {
            snowman.health =100;
            cout << "Snowman health is full!" << endl;
        }
        else {
        cout << "Snowman froze and has " << snowman.health << " hp." << endl;
        }
    }
    else {
        cout << "Incorrect action!" << endl;
    }
    } while (x!=0);
    return x;
}


int main() {
Snowman snowman(100);
Temperature temperature;

while(func(snowman)!=0) {
temperature.interact(snowman);
}

return 0;
}