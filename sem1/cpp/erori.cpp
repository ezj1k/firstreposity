#include <iostream>
#include <stdexcept>
using namespace std;

// void logic(int a, int b) {  //ceva nu este corect logic
//     if (b==0) {
//         throw logic_error("Diviziune la 0");
//     }
//     cout << a/b << endl;
// }
// try {divide(21, 0);}
// catch(const logic_error& e){cout e.what()}

// void domain(float x) { //in afara domeniului de lucru (-1 pentru radical)
//     if(x<0) {
//         throw domain_error("numar negativ");
//     }
//     cout << sqrt(x);
// }
// try{domain(-1);}
// catch(const domain_error& e){cout << e.what();}

// void invalid(int age){   // un argument introdus nu este valid
//     if(age<0 || age>150) {
//         throw invalid_argument("varsta imposibila");
//     }
//     cout << age;
// }
// try{invalid(-1);}
// catch(const invalid_argument& e){cout << e.what;}

// void length(int size){   // lungimea depaseste limita permisa
//     if(size>1000) {
//         throw length_error("dimensiune depasita");
//     }
// int arr[size];
//     cout << arr.size();
// }
// try{length(2000);}
// catch(const length_error& e){cout << e.what;}

// void out(int arr, int index){   // cand se apeleaza un element din afara limitelor unui container(de ex arr)
//     if(index>arr.size()) {
//         throw out_of_range("dimensiune depasita");
//     }
//     cout << arr[index];
// }
// try{out(2000);}
// catch(const out_of_range& e){cout << e.what;}

// void run(){   // eroare de rulare(in timpul executiei)
//         throw runtime_error("eroare");
// }
// try{run();}
// catch(const runtime_error& e){cout << e.what;}

// void range(float x){   // in afara intervalului acceptabil 
//     if(x<=0) {
//         throw range_error("dimensiune depasita");
//     }
//     cout << sqrt(x);
// }
// try{out(-1);}
// catch(const range_error& e){cout << e.what;}

// void over(int a, int b){   // un calcul produce o valoare ce depășește limita maximă a unui tip de date.
//     if(a>INT_MAX-b) {
//         throw overflow_error("dimensiune depasita");
//     }
//     cout << a+b;
// }
// try{out(INT_MAX, 1);}
// catch(const overflow_error& e){cout << e.what;}

// void under(int a, int b){   // un calcul produce o valoare ce depășește limita minima a unui tip de date.
//     if(a<INT_MIN+b) {
//         throw underflow_error("dimensiune depasita");
//     }
//     cout << a-b;
// }
// try{out(INT_MIN, 1);}
// catch(const underflow_error& e){cout << e.what;}

int main() {

    return 0;
}