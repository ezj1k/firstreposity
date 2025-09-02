from functii import introducere_date, afisare_date, cauta_fara_copii

def meniu():
    while True:
        print("1. Introducere date")
        print("2. Vizualizare date")
        print("3. Vizualizare angazati fara copii")
        print("4. Stop")
        optiune = input("Alegeți o opțiune: ").strip()

        if optiune == "1":
            introducere_date()
        elif optiune == "2":
            afisare_date()
        elif optiune == "3":
            cauta_fara_copii()
        elif optiune == "4":
            print("Iesire din meniu")
            break
        else:
            print("Eroare, mai incearca")

meniu()
