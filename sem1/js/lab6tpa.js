const imagine = document.getElementById("imagine");
const money = document.getElementById("money");
const amount = document.getElementById("amount");
const buy = document.getElementById("buy");
const restart = document.getElementById("restart");
const cost = document.getElementById("cost");

let obiect = JSON.parse(localStorage.getItem("obiect")) || {
    money: 0,
    amount: 1,
    cost: 5,
    timeInterval: 1000, // timpul inițial de adunare a banilor (în milisecunde)
    autoIncome: null // va ține referința la intervalul de timp
};

function update() {
    money.innerText = obiect.money;
    cost.innerText = obiect.cost;
    amount.innerText = obiect.amount;
}

// Funcția care adaugă bani automat
function autoAddMoney() {
    obiect.money += obiect.amount;
    localStorage.setItem("obiect", JSON.stringify(obiect));
    update();
}

// Funcția care pornește adunarea automată a banilor
function startAutoIncome() {
    // Dacă există un interval activ, îl anulăm
    if (obiect.autoIncome !== null) {
        clearInterval(obiect.autoIncome);
    }

    // Pornim un nou interval pentru adunarea automată de bani
    obiect.autoIncome = setInterval(autoAddMoney, obiect.timeInterval);
}

// Funcția de upgrade
function tobuy() {
    if (obiect.money >= obiect.cost) {
        obiect.money -= obiect.cost;
        obiect.cost *= 2;
        obiect.amount++;
        
        // Reducem intervalul de timp pentru adunarea banilor
        if (obiect.timeInterval > 500) { // Setăm o limită minimă pentru interval
            obiect.timeInterval -= 200; // Reducem intervalul cu 200 ms
        }

        // Salvăm obiectul în localStorage
        localStorage.setItem("obiect", JSON.stringify(obiect));
        update();
        
        // Reîncepem adunarea automată cu noul interval
        startAutoIncome();
    } else {
        alert("Nu ai destui bani!");
    }
}

// Funcția pentru restart
function torestart() {
    obiect.money = 0;
    obiect.amount = 1;
    obiect.cost = 5;
    obiect.timeInterval = 1000; // Resetăm intervalul de timp la valoarea inițială
    localStorage.setItem("obiect", JSON.stringify(obiect));
    update();

    // Oprim adunarea automată
    if (obiect.autoIncome !== null) {
        clearInterval(obiect.autoIncome);
        obiect.autoIncome = null;
    }
}

// Pornim adunarea automată la început
startAutoIncome();

imagine.addEventListener("click", autoAddMoney);
restart.addEventListener("click", torestart);
buy.addEventListener("click", tobuy);