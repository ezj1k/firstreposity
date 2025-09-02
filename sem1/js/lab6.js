const imagine = document.getElementById("imagine")
const money = document.getElementById("money")
const amount = document.getElementById("amount")
const buy = document.getElementById("buy")
const restart = document.getElementById("restart")
const cost = document.getElementById("cost")

function update() {
    money.innerText=`${obiect.money}`;
    cost.innerText=`${obiect.cost}`;
    amount.innerText=`${obiect.amount}`;
}

let obiect = JSON.parse(localStorage.getItem("obiect")) || {
    money: 0,
    amount: 1,
    cost: 5,
}
update();



function toaddmoney() {
    obiect.money+=obiect.amount;
    localStorage.setItem("obiect", JSON.stringify(obiect));
    update();
}

function tobuy() {
    if(obiect.amount<5) {
        if(obiect.money>=obiect.cost) {
        obiect.money-=obiect.cost;
        obiect.cost*=2;
        obiect.amount++;
        localStorage.setItem("obiect", JSON.stringify(obiect));
        update();
        }
        else{
        alert("nu iti ajung bani");
        }
    }
    else{
        cost.innerText=`MAX`;
        localStorage.setItem("obiect", obiect);
        update();
        alert("Sa ajuns la maxim!");
        
    }
}

function torestart() {
    obiect.money=0;
    obiect.amount=1;
    obiect.cost=5;
    localStorage.setItem("obiect", JSON.stringify(obiect));
    update();
}


imagine.addEventListener("click", toaddmoney)
restart.addEventListener("click", torestart)
buy.addEventListener("click", tobuy)