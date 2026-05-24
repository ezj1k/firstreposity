const input = document.getElementById("input")
const clear = document.getElementById("clear")
const output = document.getElementById("output")

function clearing(){
    input.value="";
    output.innerText="";
}

function outputing(){
    console.log("a")
    const numere=[];
    const operatii=[];
    regex=/\d+|[+\-\*\/]/g;
    let input1=input.value;

    const elemente=input1.match(regex);
    if(!elemente) {
        alert("Introdu ceva");
        input.value=""
    }

    for(let i=0; i<elemente.length;i++){
        if(!isNaN(elemente[i])){
            numere.push(Number(elemente[i]));
        }
        else if (/[+\-\*\/]/.test(elemente[i])) {
            operatii.push(elemente[i]);
        } 
        else {
            alert("INTRODU NUMERE SI ACEASI OPERATI!");
            input.value=""
        }
    }

    let diferit = false;
    for (let i = 0; i < operatii.length - 1; i++) {
        if (operatii[i] !== operatii[i + 1]) {
            diferit = true;
            break;
        }
    }

    if (diferit === true) {
        alert("Scrie aceeasi operatie peste tot!");
        input.value="";
    }
    else {
        let result;
        if (operatii[0] === "+") {
            result = numere.reduce((acc, cv) => acc + cv, 0);
        } 
        else if (operatii[0] === "-") {
            result = numere.reduce((acc, cv) => acc - cv);
        } 
        else if (operatii[0] === "*") {
            result = numere.reduce((acc, cv) => acc * cv, 1);
        } 
        else if (operatii[0] === "/") {
            result = numere.reduce((acc, cv) => acc / cv);
        }
        historyplus(result);
    }

    function historyplus(result) {
        output.innerHTML += `<p>${input.value} = ${result}</p>`;
    }
}

clear.addEventListener("click", clearing)
input.addEventListener("change", outputing)