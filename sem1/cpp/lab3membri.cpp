#include <iostream>

using namespace std;

class Complex {
private:
    int a, b;  // părțile reale și imaginare ale numărului complex
public:
    Complex() {
        a = 0;
        b = 0;
    }

    Complex(const Complex& q) {
        a = q.a;
        b = q.b;
    }

    Complex(double real, double imag) {
        a = static_cast<int>(real);  // Convertim valorile de tip double în int
        b = static_cast<int>(imag);
    }

    friend istream& operator>>(istream& in, Complex& z);
    friend ostream& operator<<(ostream& out, const Complex& z);
    friend Complex& operator-=(Complex& c1, const Complex& c2);
    friend Complex operator++(Complex& c, int);  // postfix
    friend Complex& operator++(Complex& c);      // prefix
    friend bool operator>=(const Complex& c1, const Complex& c2);
    friend Complex operator/(const Complex& c1, int x);
    friend Complex operator/(const Complex& c1, double x);

    int getA() const { return a; }
    int getB() const { return b; }

    void afisare() const {
        cout << *this << endl;
    }

    ~Complex() {
        clog << "Stergere" << endl;
    }
};

    istream& operator>>(istream& in, Complex& z) {
        in >> z.a >> z.b;
        return in;
    }

    ostream& operator<<(ostream& out, const Complex& z) {
        out << z.a << " + " << z.b << "i";
        return out;
    }

    Complex& operator-=(Complex& c1, const Complex& c2) {
        c1.a -= c2.a;
        c1.b -= c2.b;
        return c1;
    }

    Complex operator++(Complex& c1, int) { // postfix ++a ++b
        Complex temp = c1;
        ++c1;
        return temp;
    }

    Complex& operator++(Complex& c1) { // prefix ++a ++b
        c1.a++;
        c1.b++;
        return c1;
    }

    bool operator>=(const Complex& c1, const Complex& c2) {
        return (c1.a >= c2.a && c1.b >= c2.b);
    }

    Complex operator/(const Complex& c1, int x) {
        return (c1 / static_cast<double>(x));
    }

    Complex operator/(const Complex& c1, double x) {
        return Complex(static_cast<double>(c1.a) / x, static_cast<double>(c1.b) / x);
    }


int main() {
    Complex z1,z2,z3,z4;
    cout << "Introdu a, b: ";
    cin >> z1;
    cout << "Afisare: " << z1 << endl;
    
    cout << "Introdu a, b: ";
    cin >> z2;
    cout << "Afisare: " << z2 << endl;

    z1-=z2;
    cout << "rezultatul scaderii: " << z1 << endl;

    z3 = z1++;
    cout << "valoarea lui z1 inainte de incrementare: " << z3 << endl;
    cout << "rezultatul incrimentarii: " << z1 << endl;

    ++z1;
    cout << "rezultatul incrimentarii prefixe: " << z1 << endl;

    if (z1>=z2) {
        cout << "z1 mai mare sau egal" << endl;
    }
    else {
        cout << "z2 mai mare" << endl;
    }

    z4 = z1 / 2;
    cout << "rezultatul impartirii: " << z4 << endl;

    // Exemplu de împărțire a unui număr complex la un număr real
    Complex z5 = z1 / 2.0;
    cout << "Impartire cu numar real: " << z5 << endl;

    Complex z6 = z1;
    cout << "atribuire lui z6: " << z6 << endl;

    return 0;
}