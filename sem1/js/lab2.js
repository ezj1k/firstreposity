let x=0;
const text=document.querySelector('#text');
const button1=document.querySelector('#button1');
const button2=document.querySelector('#button2');
const button3=document.querySelector('#button3');
const button4=document.querySelector('#button4');
const history=document.querySelector('#history');

    button1.onclick = Add2;
    button2.onclick = Add6;
    button3.onclick = Substract1;
    button4.onclick = Substract3;
    function update() {
        xText.innerText = x;
        console.log(history.innerText);
        history.innerText+=" "+x;
      }

function Add2() {
    x+=2;
    update();
}
function Add6() {
    x+=6;
    update();
}
function Substract1() {
    x-=1;
    update();
}
function Substract3() {
    x-=3;
    update();
}