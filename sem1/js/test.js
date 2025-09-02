let lunadic = {};

        const selectMonth = document.getElementById("selectMonth");
        const inputText = document.getElementById("inputText");
        const submitButton = document.getElementById("submitButton");
        const output = document.getElementById("output");

function LunaCurenta() {
const luni= ["Ianuarie", "Februarie", "Martie", "Aprilie", "Mai", "Iunie", "Iulie", "August", "septembrie", "Octombrie", "Noiembrie", "Decembrie"]
const lunaAcum=new Date().getMonth();
document.getElementById("selectMonth").value = luni[lunaAcum];
}

        function actualizeazaTabel() {
            const tbody = output.querySelector("tbody");

            for (const luna in lunadic) {
                const tr = document.createElement("tr");

                const tdLuna = document.createElement("td");
                tdLuna.textContent = luna;

                const tdText = document.createElement("td");
                tdText.textContent = lunadic[luna];

                tr.appendChild(tdLuna);
                tr.appendChild(tdText);
                tbody.appendChild(tr);
            }
        }

        selectMonth.addEventListener("change", ()=> {
            const lunaSelectata = selectMonth.value;
            inputText.value = lunadic[lunaSelectata] || "";
        });

        submitButton.addEventListener("click", ()=> {
            const lunaSelectata = selectMonth.value;
            const textIntroduc = inputText.value;
            
            if (textIntroduc !== "") {
                lunadic[lunaSelectata] = textIntroduc;
                actualizeazaTabel();
            }
        });

        window.onload=LunaCurenta();