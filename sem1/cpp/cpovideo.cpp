#include <iostream>
#include <fstream> // [2]
#include <cstring> // [8]
#include <cmath> //[8]
using namespace std;
// [6] enum
// enum Options {
//     close=4, //common starts with 0, now its 4
//     open, //its 5
//     wait=12, // its 12
//     del, // its 13
// };
// struct File {
// float weight;
// string name;
// Options options;
// } [/6]
//---------------------------
//---int sum(int a, int b); [5] functii [4] pointeri si adrese
/*int min(int* arr, int len) { 
int minim=*arr;
for(int i=0;i<len;i++) {
if(minim>*(arr+i)) {
minim=*(arr+i);
}}}  [/5] [/4]*/
//================================
//[1]structuri
// struct Point {
// int x,y;
// };
// struct Tree {
//     string name;
//     int ages;
//     bool is_alive;
//     float height;
//     Point place;

//     void pokaz() {
//         cout << name << " - imea";
//     }
// };[/1]
//--------------------------------------
//[7] Iskliucenia
// void devide(float a, float b) {
//     if(b==0) {throw 100;} //vidast error
// else {cout << (a/b);}}
//[/7]
//---------------------------------------

int main() {
    
    //dinamic memory
    // int *nums = new int[3]; //sozdal novii masiv(videlil memory pod 3 elementa)
    // nums[0]=45;
    // cout << nums[0] << endl;
    // delete[] nums;
    //========================

//[3] pointeri i adresi
/* //links(ssilki)
int num=10;
int &a = num;  // zapisivaem adres num v &a chtob mojno bilo rabotati s nim;
//&a - adres catre num; a budet = num
a=15; // num toje stanet 15;
cout << &num << " = " << num => adres = 10
cout << &a << " = " << a => adres = 10
------------------------
//pointers(ukazateli)
int val=12;
int* ptrval=&val; //zapisivaem link val v ptrval chtob mojno bilo rabotati s nim;
cout << &val << "-" << val; // adres - 12 // adres - znacenie
cout << ptrval << "-" << *ptrval; // adres - 12 // pointer(&val) - pointer linka(*&val = val)(znacenie)
*ptrval=20; // &val toje budet = 20
ptrval=nullptr; //16 zerouri
int arr[]={1,2,3};
min(arr,5); [/3]*/
//-----------------------------------      
                                                      //обьектно ориентированое программирование
//[2] robota s failami
// ofstream file("test.txt", ios_base::out);
// if(file.is_open()) {
// file << "Hello World";
// file.close();
//}
// ifstream file("test.txt");
// if(file.is_open()) {
// // string temp;
// // file >> temp; only pervoe slovo
// char temp[100];
// file.getline(temp, 100); perie 100 simvolov
// file.close();
// }  [/2]
//-----------------------------

//[1] struct
// Tree dub;
// dub.name="dub";
// dub.ages=100;
// dub.height=12.5;
// dub.is_alive=true;

// Tree elka;
// elka.name="elka";
// elka.ages=12;
// cout << dub.name << " - imea;";
// elka.pokaz();
// dub.place.x=10;
// dub.place.y=32;  [/1]
//=======================================

//[6] enum
// File my_file;
// my_file.weight=1.5f;
// my_file.name="text.txt";
// my_file.options=Options::close;
// cout << my_file.options;
// if (my_file.options==Options::close) {
//     cout << "file is close";
// } [/6]
//---------------------------------------------

//[7] Iskliucenia
// try {
// devide(5.2f, 0.0f);
// }
// catch(int err) {
// if (err==100) {cout << "Eroare la impartire la 0";}
// } [/7]
//============================================

//[8] vstroenie functii
// //cstring
// string str1="Hello";
// string str2="World";
// str1.append(str2); // a adauga (+) <=> str1+str2
// str1.pop_back(); // sterge last symbol
// str1.push_back('!'); // adauga symbol
// str1.length(); // or .size() - lungimea
// str1.resize(5); // face sa fie mximum 5 elemente
// //cmath
// pow(2,3); // ridicare la putere
// abs(-3); //modulul
// sin(1); // sinus
// cos(1); //cosinus
// sqrt(4); //radical din 2
// ceil(1.28f); // rotunjire in sus (2)
// floor(1.78f); // rotunjire in jos (1)
// round(1.45f); // rotunjire la cel mai apropiat intreg (1); round(1.55f) = (2)



return 0;
}