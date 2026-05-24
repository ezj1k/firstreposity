#include <iostream>
#include <algorithm>
#include <string>

using namespace std;

int suma(int a, int b) {
    string x="";
    if(b>a) {
        int temp = a;
        a=b;
        b=temp;
    }
string as = to_string(a);
string bs = to_string(b);
for (int i=0;i<as.size();i++) {
    for (int j=0;j<bs.size();j++) {
        if(as[i]==bs[j]) {
            x+=bs[j];
        }
    }
}
for (int i=0; i<x.size(); i++) {
    for(int j=i+1;j<x.size();j++)
    if(x[i]==x[j]){
        x[j]=0;
    }
}
int q=stoi(x);
    return q;
}

int main() {
int a,b;
cin >> a >> b;
cout << suma(a,b);

    return 0;
}