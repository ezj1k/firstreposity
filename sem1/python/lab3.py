greet_user = lambda name : print('Hello My Dear, ', name) #in functia greet_user se declara o variabila (name) care este folosita apoi intr-o afisare, ca parametru.
user_name = input("What is your name? ") #introduce o oarecare variabila
greet_user(user_name) #declara functia (greet_user) folosind ca parametrul (user_name) 
#What is your name? Name
#Hello My Dear, Name

key=lambda x: x[1]
reverse=False
lista = [(3, 11), (1, 7), (7, 8), (16, 88), (23, 15)]
sortat=sorted(lista, key=key,reverse=reverse)
print(sortat)

noi=100
print("Au fost aduse haine noi? ")
adus = input()
haine=lambda vechi: vechi+noi
if (adus=="Da"):
    print("acum sunt ",haine(500), " haine")
else:
    print("sunt tot atatea haine")

#fara parametri
def Hello():
    print("sa executat functia Hello()")
Hello()
#cu parametri
def Evaluare(punctaj):
    if punctaj <=20:
        print(f"{punctaj} nu coincide cu nota trecatoare")
    else:
        print(f"{punctaj} coincide cu nota trecatoare")
Evaluare(50)
#cu valori predefinite pentru parametri
def Nota(mark=5, name="Alex"):
    if mark<=4:
        print(f"{name} are restanta")
    else:
        print(f"{name} nu are restanta")
Nota()
Nota(2)
Nota(4,"Mihai")
#care returneaza
def add(a, b):
    return a + b
print(add(4,5))
#care nu foloseste return
def summa(a, b):
    print(a+b)
summa(5,6)

#pentru a determina daca un numar este par si impar
def divide_numbers(a):
    if a%2==0:
        print(a, " - este nr par")
    else:
        print(a, " - nu este nr par")
divide_numbers(4)
#pentru a returna o valoare
def suma(a, b):
    return (a+b)
x = suma(5,6)
print(x)
#pentru a forma o lista cu prezenta
def listaa(nr_elevi):
    for i in range(1,nr_elevi+1):
        print("scrie numele: ")
        nume=input()
        print("scrie prezent sau nu: ")
        prezent=input()
        if prezent=="+":
            print(i+1, ") ", nume, " prezent")
        else:
            print(i+1, ") ", nume, " absent")
listaa(5)
        



# 1)    1)	Explicați ce a fost realizat in urmatorul exemplu:
# 2)	Sortați o listă de 7 tupluri a câte 2 elemente, după cel de-al doilea element. Pentru a rezolva această problemă, utilizați funcția sorted(iterable, key=key, reverse=reverse) și creați o expresie-lambda pentru cel de-al 2-lea parametru
# De exemplu, dacă lista constă din:
# [(3, 11), (1, 7), (7, 8), (16, 88), (23, 15)]
# Rezultatul ar trebui să fie acesta:
# [(1, 7), (7, 8), (3, 11), (23, 15), (16, 88)]
# 3)	Creați propria funcție-lambda pentru a rezolva o problemă. Explicați profesorului principiul creării și utilizării acesteia.
# 4)	Definiți câte o funcție de următoarele tipuri:
# a.	Fără parametri, cu parametri și cu valori predefinite pentru parametri;
# b.	Care returnează un rezultat și care nu folosește return;
# 5)	Apelați funcțiile pentru a rezolva unele probleme. Explicați cum sunt apelate funcțiile ținând cont de modul în care le-ați definit.
