const units = {
  length: ["meters", "kilometers", "millimeters"],
  weight: ["grams", "kilograms", "pounds"],
  temperature: ["celsius", "fahrenheit", "kelvin"]
};

let currentCategory = "length";

// Elemente HTML salvate în constante
const inputValueEl = document.getElementById("inputValue");
const fromUnitEl = document.getElementById("fromUnit");
const toUnitEl = document.getElementById("toUnit");
const resultEl = document.getElementById("result");

const btnLength = document.getElementById("btnLength");
const btnWeight = document.getElementById("btnWeight");
const btnTemp = document.getElementById("btnTemp");
const btnConvert = document.getElementById("btnConvert");
const btnDarkMode = document.getElementById("btnDarkMode");

// La încărcarea paginii
// window.onload = () => {
  setCategory(currentCategory); //mereu e length la refresh
  
  const saved = localStorage.getItem("lastResult");
  if (saved) {
    resultEl.innerText = saved;
  }

  if (localStorage.getItem("darkMode") === "true") {
    document.body.classList.add("dark"); //verifica daca este dark sau nu
  }
// };

// Setează categoria selectată
function setCategory(category) {
  currentCategory = category; //salveaza caegoria curenta

  fromUnitEl.innerHTML = "";
  toUnitEl.innerHTML = ""; //golire

  units[category].forEach(unit => {
    fromUnitEl.add(new Option(unit, unit));
    toUnitEl.add(new Option(unit, unit)); //completare cu noi
  });

  fromUnitEl.selectedIndex = 0;
  toUnitEl.selectedIndex = 1;
}

// Funcția principală de conversie
function convert() {
  const value = parseFloat(inputValueEl.value); //transforma din int in float
  const from = fromUnitEl.value;
  const to = toUnitEl.value;

  if (isNaN(value)) {
    resultEl.innerText = "Introdu o valoare validă.";
    return;
  }

  let result = "";
  switch (currentCategory) {
    case "length":
      result = convertLength(value, from, to);
      break;
    case "weight":
      result = convertWeight(value, from, to);
      break;
    case "temperature":
      result = convertTemperature(value, from, to);
      break;
  }

  const output = `Rezultat: ${value} ${from} = ${result} ${to}`;
  resultEl.innerText = output;
  localStorage.setItem("lastResult", output);
}

// Conversii
function convertLength(value, from, to) {
  const toMeters = { meters: 1, kilometers: 1000, millimeters: 0.001 };
  return (value * toMeters[from] / toMeters[to]).toFixed(4);
}

function convertWeight(value, from, to) {
  const toGrams = { grams: 1, kilograms: 1000, pounds: 453.592 };
  return (value * toGrams[from] / toGrams[to]).toFixed(4);
}

function convertTemperature(value, from, to) {
  let celsius;
  if (from === "celsius") celsius = value;
  else if (from === "fahrenheit") celsius = (value - 32) * 5 / 9;
  else if (from === "kelvin") celsius = value - 273.15;

  let result;
  if (to === "celsius") result = celsius;
  else if (to === "fahrenheit") result = celsius * 9 / 5 + 32;
  else if (to === "kelvin") result = celsius + 273.15;

  return result.toFixed(2);
}

// Tema dark
function toggleDarkMode() {
  document.body.classList.toggle("dark");
  const isDark = document.body.classList.contains("dark");
  localStorage.setItem("darkMode", isDark);
}

// Legare evenimente pe butoane
btnLength.addEventListener("click", () => setCategory("length"));
btnWeight.addEventListener("click", () => setCategory("weight"));
btnTemp.addEventListener("click", () => setCategory("temperature"));
btnConvert.addEventListener("click", convert);
btnDarkMode.addEventListener("click", toggleDarkMode);
