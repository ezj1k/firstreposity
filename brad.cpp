#include <iostream>

using namespace std;

int main() {
srand(time(NULL));
string arr[20][20];
int size=16;
int c=0;
int x,y;

for(int i=0; i<5; ++i){
    for(int j=0; j<=size; ++j){
        if(j+i==size/2+c*3 || j-i==size/2-c*3){
        arr[i][j]="*";
        }
        else{arr[i][j]=" ";}
        if(i==4){
            for(int j=i; j<=size-i; ++j){
                arr[i][j]="*";
            }
        }
    }
}
c++;

for(int i=5; i<10; ++i){
    for(int j=0; j<=size; ++j){
        if(j+i==size/2+c*3 || j-i==size/2-c*3){
        arr[i][j]="*";
        }
        else{arr[i][j]=" ";}
        if(i==9){
            for(int j=2; j<=size-2; ++j){
                arr[i][j]="*";
            }
        }
    }
}
c++;

for(int i=10; i<15; ++i){
    for(int j=0; j<=size; ++j){
        if(j+i==size/2+c*3 || j-i==size/2-c*3){
        arr[i][j]="*";
        }
        else{arr[i][j]=" ";}
        if(i==14){
            for(j=0; j<=size; ++j){
                arr[i][j]="*";
            }
        }
    }
}

for(int i=15; i<18; ++i){
    for(int j=0; j<=16; ++j){
        if(j==7 || j==8 || j==9 || j==10 || j==6){
            arr[i][j]="*";
        }
        else {
            arr[i][j]=" ";
        }
    }
}

for(int i=0; i<20; ++i){
    for(int j=0; j<20; ++j){
        cout << arr[i][j];
    }
    cout << endl;
}
//--------------------------------------------
cout << "vrei jucarii?(da/nu)" << endl;
string raspuns;
cin >> raspuns;
while(raspuns!="da" && raspuns!="nu"){
    cout << "raspuns incorect(scrie da sau nu)" << endl;
    cin >> raspuns;
}
if(raspuns=="da"){
    cout << "cate jucarii vrei?" << endl;
    int nr;
    cin >> nr;
    while(nr<=0){
        cout << "raspuns incorect(scrie un numar intreg mai mare ca 0)" << endl;
        cin >> nr;
    }
    for(int k=0; k<nr; ++k){
        bool plasat=false;
        while(!plasat){
            x=1+rand()%13;
            y=2+rand()%14;
            if(arr[x][y]!="*"){
                for(int t=0;t<y;++t){
                    if(arr[x][t]=="*"){
                        for(int t=y+1;t<16;++t){
                            if(arr[x][t]=="*"){
                                if(arr[x-1][y]!="*" || arr[x+1][y]!="*") {
                                    arr[x][y]="*";
                                    plasat=true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

for(int i=0; i<20; ++i){
    for(int j=0; j<20; ++j){
        cout << arr[i][j];
    }
    cout << endl;
}
    return 0;
}

/* 
i0  j8  j+-i=8
i1  j7-9
i2  j6-10
i3  j5-11
i4  j4-12 j5-11 j6-10
---------
i5  j6-10
i6  j5-11
i7  j4-12
i8  j3-13
i9  j2-14
----------
i10 j4-12
i11 j3-13
i12 j2-14
i13 j1-15
i14 j0-16

*/