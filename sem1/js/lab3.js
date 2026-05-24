const buton=document.querySelector('#buton');
buton.addEventListener("click", creazaParagrafe());

function creazaParagrafe() {
    const n = document.getElementById('numarParagrafe').value;
    const container = document.getElementById('container');
    container.innerHTML = '';

      for (let i = 0; i < n; i++) {
        const paragraf = document.createElement('p');
        paragraf.innerHTML = `
          <input type="number" class="input-numar">
        `;
        container.appendChild(paragraf);
      }

      document.getElementById('Sum').style.display = 'block';
      document.getElementById('rezultatulSumei').style.display = 'none';
    }

    function calculeazaSuma() {
      const inputs = document.querySelectorAll('.input-numar');
      let suma = 0;

	for (let i = 0; i < inputs.length; i++) {
	  suma += Number(inputs[i].value);
	}

      document.getElementById('suma').textContent = suma;
      document.getElementById('rezultatulSumei').style.display = 'block';
    }