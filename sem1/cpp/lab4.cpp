#include <iostream>
#include <stdexcept>  // Pentru gestionarea excepțiilor

using namespace std;

// Clasa șablon Complex
template <typename T>
class Complex {
private:
    T a, b;  // părțile reale și imaginare ale numărului complex
public:
    Complex() : a(0), b(0) {}

    Complex(const Complex& q) : a(q.a), b(q.b) {}

    Complex(T real, T imag) : a(real), b(imag) {}

    friend istream& operator>>(istream& in, Complex& z) {
        in >> z.a >> z.b;
        return in;
    }

    friend ostream& operator<<(ostream& out, const Complex& z) {
        out << z.a << " + " << z.b << "i";
        return out;
    }

    Complex& operator-=(const Complex& other) {
        a -= other.a;
        b -= other.b;
        return *this;
    }

    Complex operator++(int) { // postfix ++a ++b
        Complex temp = *this;
        ++*this;
        return temp;
    }

    Complex& operator++() { // prefix ++a ++b
        a++;
        b++;
        return *this;
    }

    bool operator>=(const Complex& comp) const {
        return (this->a >= comp.a && this->b >= comp.b);
    }

    Complex operator/(T x) const {
        if (x == 0) {  // Prevenim diviziunea cu zero
            throw invalid_argument("Divizarea la zero nu este permisa!");
        }
        return Complex(a / x, b / x);
    }

    Complex& operator=(const Complex& var) {
        if (this != &var) {
            a = var.a;
            b = var.b;
        }
        return *this;
    }

    T getA() const { return a; }
    T getB() const { return b; }

    void afisare() const {
        cout << *this << endl;
    }

    ~Complex() {
        clog << "Stergere" << endl;
    }
};

// Funcție șablon pentru a lucra cu obiecte de tip Complex
template <typename T>
void operatiiComplex(Complex<T>& z1, Complex<T>& z2) {
    cout << "Introduceti a, b pentru primul numar complex: ";
    cin >> z1;

    cout << "Introduceti a, b pentru al doilea numar complex: ";
    cin >> z2;

    // Exemplu de scădere
    z1 -= z2;
    cout << "rezultatul scaderii: " << z1 << endl;

    // Exemplu de incrementare postfix
    Complex<T> z3 = z1++;
    cout << "valoarea lui z1 inainte de incrementare: " << z3 << endl;
    cout << "rezultatul incrementarii: " << z1 << endl;

    // Exemplu de incrementare prefix
    ++z1;
    cout << "rezultatul incrementarii prefixe: " << z1 << endl;

    // Comparare
    if (z1 >= z2) {
        cout << "z1 este mai mare sau egal cu z2." << endl;
    } else {
        cout << "z2 este mai mare." << endl;
    }

    T x;
    cout << "Introduceti un numar pentru impartire: ";
    cin >> x;
    // Exemplu de împărțire
    Complex<T> z4 = z1 / x;
    cout << "rezultatul impartirii: " << z4 << endl;

    // Exemplu de atribuire
    Complex<T> z5 = z1;
    cout << "atribuirea lui z5: " << z5 << endl;
}

int main() {
    Complex<int> z1, z2, z3;
    Complex<double> z4, z5;
try{
    cout << "Operatii pe numere complexe de tip int:" << endl;
    operatiiComplex(z1, z2);

    cout << "\nOperatii pe numere complexe de tip double:" << endl;
    operatiiComplex(z4, z5);
}
catch(const exception& e){
    cout << "Eroare: " << e.what() << endl;
}

    return 0;
}
