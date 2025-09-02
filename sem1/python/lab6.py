

# Scrieți un program în Python care va verifica dacă șirul introdus (în input()) de utilizator este un număr de telefon valid din Moldova: este permis să fie introdus un număr cu un cod de țară: 0037399999999 sau +37399999999 (unde 9 este orice cifră din intervalul 0 -9) sau numărul în formă prescurtată – orice 8 cifre – 99999999 (de exemplu 79886755).
# Adică, utilizatorul poate introduce 0037378665544 sau +37378665544 sau 78665544 - și acest lucru va fi corect.
# Pentru a verifica formatul numărului de telefon, utilizați expresii regulate și instrucțiunea try..except - pentru a gestiona posibilele erori la introducerea datelor.
# De asemenea, utilizați u ciclu și readuceți utilizatorul în modul de introducere până când introduce corect numărul solicitat. Dacă introducerea este corectă, afișați numărul și un mesaj de laudă utilizatorului.




import re

#telefon = r"^(00373|\+373)?\d{8}$"
regex=r"^([1]\d)|20$"

while True:
    try:
        numar = input("Introdu numar de telefon valid din Moldova: ")
        if re.match(regex, numar):
            print(f"Numarul de telefon {numar} este valid! Mesaj de lauda utilizatorului")
            break
        else:
            print("Numarul este invalid!(structura: 00373nnnnnnnn, +373nnnnnnnn sau nnnnnnnn, n = orice numar de la 0 la 9)")
    except:
        print("Ceva a mers gresit, Te rog sa încerci din nou!")
