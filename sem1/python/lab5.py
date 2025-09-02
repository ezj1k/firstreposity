#sarcina 1
i = sum = 0 #se formeaza 2 variabile cu valoarea 0
while i <= 4: #atat timp cat i e mai mic sau egal ca 4
    sum += i #variabila sum va aduna valoarea curenta lui i
    i = i+1 #apoi i se incrementeaza, si asa pana nu devine mai mare ca 4, dupa ce ciclul se opreste
print(sum) # se afiseaza suma
#se va afisa "10"

for char in 'PYTHON STRING': #prin ciclu se parcurg toate caracterele din stringul prezent
  if char == ' ': #se verifica daca caracterul prezent nu este spatiu liber
      break #daca da, atunci se opreste ciclul
  print(char, end='') #se afiseaza caractereul care este prezent, si la sfarsit se face ca ultimul element sa fie nimic(sa inlocuiasca \n)
  if char == 'O': #se verifica daca caracterul prezent nu este litera O
      continue #daca da, atunci se ruleaza urmatorul ciclu(adica va sari peste ciclul dat, si va merge mai departe)
  print('*', end='') #se afiseaza o steluta
  #se va afisa "P*Y*T*H*ON*"
#---------------------------------------
#sarcina 2
#a) Scrieți un cod care prelucrează valoarea introdusă de utilizator și îi afișează un mesaj folosind instrucțiunea if...elif...else.
x=input("Scie un cuvant: ")
if (x=="cuvant"):
    print("Nu am avut in vedere in sensul direct!")
elif(x=="Mama"):
    print("Este primul cuvant al majoritatea oamenilor")
else:
    print("Cuvantul tau este ", x)

#b)Parcurgeți toate valorile a trei variabile de tip: listă (list), dicționar (dict) și text (str), și creați  un contor pentru a număra de câte ori apare o anumită valoare în colecții (stabiliți valoarea în mod independent). Pentru a rezolva problema, utilizați câte un ciclu while sau for (care considerați că este mai optim) și o instrucțiune condiționată. Produceți și analizați rezultatul.
lista=[1,2,4,3,98,34,2,1]
numarul_se_repeta=0
for numar in lista:
    if numar==1:
        numarul_se_repeta+=1
print("numarul se repeta de ", numarul_se_repeta, " ori.")

dictionar={"0": "valoare0", "1": "valoare1", "2": "valoare2", "3": "valoare1"}
valoarea_se_repeta=0
for cheie, valoare in dictionar.items():
    if (valoare=="valoare1"):
        valoarea_se_repeta+=1
print("valoarea se repeta de ", valoarea_se_repeta, " ori.")

catecharactere=0
for character in x:
    if(character=="a"):
        catecharactere+=1
print("stringul are: ", catecharactere, " caractere.")

#sarcina 3
# a)	Definiți 3 funcții: una pentru a adăuga denumirea unui produs în listă (validați ca în cazul când produsul deja este în listă – să nu se facă adăugarea lui repetată, avertizându-l pe utilizator despre aceasta); una pentru a șterge denumirea unui produs din listă (în baza numărului de ordine din listă și în baza denumirii produsului); și una pentru afișarea tuturor elementelor din lista curentă.
#b)	Creați un meniu din 4 opțiuni, din care utilizatorul poate selecta una din ele: 
# 1.	Afișarea listei curente de produse,
# 2.	Adăugarea produsului în listă, (în baza funcției input()),
# 3.	Ștergerea denumirii produsului din listă, 
# 4.	Ieșirea din regimul de editare a listei.
import functiipentrulista as functie
cumparaturi=[]
x=1
while (x):
    print("Optiunea 1: Adauga produs;")
    print("Optiunea 2: Sterge produs;")
    print("Optiunea 3: Afisarea listei;")
    print("optiunea 4: Iesirea din lista")
    optiune = input("Alege optiunea: ")

    if optiune=="1":
        functie.adaugare(cumparaturi)
    elif optiune=="2":
        functie.sterge(cumparaturi)
    elif optiune=="3":
        functie.afisare(cumparaturi)
    elif optiune=="4":
        x=0
    else:
        print("Optiune invalida, scrie din nou (1-4): ", end='')
    