let width=5;
let height=4;
let rotatie=true;
let character="#";
let m=[];
let space=" ";
let x=3;

if(!rotatie) {
    let a =height;
    height=width;
    width=a;
    x=1}

for(let i=0;i<width;i++) { 
    if(i==0 || i==width-1) {
    for(let j=0; j<height; j++) {
        m+=character;
    }
    }
    else {
            m+=character+space.repeat(width-x)+character;
        }
        
        m+="\n";
}
console.log(m+"\n");

function mypow(x) {
console.log(Math.pow(x,2,6));
}
mypow(5);