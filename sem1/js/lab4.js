const input = document.getElementById('input');
const addButton = document.getElementById('add');
const output = document.getElementById('output');
const sortedOutput = document.getElementById('sortedOutput')
const sortList = document.getElementById('sort');

let elemente = [];

function add () {
    const newEntry = Number(input.value);
    elemente.push(newEntry);
    update();
}

function update () {
    const HTML = elemente.map((value, index) => {
        return `<p style="margin: 5px" onclick="remove(${index})">Numarul: ${value}</p>`
    }).join("");
    output.innerHTML = HTML;
}

function remove (index) {
    elemente.splice(index, 1);
    update();
}

function sort () {
    elemente.sort((a, b) => a - b)
    sortedOutput.innerText = elemente;
    update();
}

addButton.addEventListener("click", add);
sortList.addEventListener("click", sort);