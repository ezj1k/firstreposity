import re

def validare_nume(nume):
    numeregex = r"[A-Za-z]{2,20}(-[A-Za]{2,20})*"
    numecorect= re.fullmatch(numeregex, nume)
    if numecorect:
        return True
    else:
        return False

def validare_departament(departament):
    departamentregex = r"[A-Za-z0-9]+( [A-Za-z0-9]+)?"
    departamentcorect = re.fullmatch(departamentregex, departament)
    if departamentcorect:
        return True
    else:
        return False

def validare_copii(nr):
    nrregex=r"[0-9]|1[0-9]"
    nrcorect= re.fullmatch(nrregex, nr)
    if nrcorect:
        return True
    else:
        return False

def introducere_date():
    while True:
        nume = input("Introduceti numele angajatului: ").strip()
        prenume = input("Introduceti prenumele angajatului: ").strip()
        departament = input("Introduceti departamentul: ").strip()
        copii = input("Introduceti numarul de copii minori: ").strip()

        if not (validare_nume(nume) and validare_nume(prenume)):
            print("Nume sau prenume invalid! (doar litere, 2-20 caractere, opțional \"-\" între ele)")
            continue
        if not validare_departament(departament):
            print("Denumirea departamentului este invalidă (doar litere/cifre, max un spațiu)")
            continue
        if not validare_copii(copii):
            print("Numărul de copii este invalid (0-19)")
            continue

        departament = departament.replace(" ", "_")
        with open("data.txt", "a") as fisier:
            fisier.write(f"{nume}\t{prenume}\t{departament}\t{copii}\n")
        print("Datele au fost salvate cu succes.")
        break

def afisare_date():
    total_copii = 0
    try:
        with open("data.txt", "r") as fisier:
            linii = fisier.readlines()
            if not linii:
                print("Nu există date salvate.")
                return
            print("\nLista angajatilor\n")
            for linie in linii:
                nume, prenume, departament, copii = linie.strip().split("\t")
                print(f"{nume} {prenume} - Departament: {departament.replace('_', ' ')}, Copii: {copii}")
                total_copii += int(copii)
            print(f"\nTOTAL copii: {total_copii}")
    except FileNotFoundError:
        print("Fisierul data.txt nu exista inca.")

def cauta_fara_copii():
    gasiti = []
    try:
        with open("data.txt", "r") as fisier:
            for linie in fisier:
                nume, prenume, departament, copii = linie.strip().split("\t")
                if copii == "0":
                    gasiti.append(f"{nume} {prenume}")
        if gasiti:
            print("\nAngajați fără copii\n")
            for persoana in gasiti:
                print(persoana)
        else:
            print("Toți angajații au cel puțin un copil.")
    except FileNotFoundError:
        print("Fișierul data.txt nu există încă.")
