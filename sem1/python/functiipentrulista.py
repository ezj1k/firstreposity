def adaugare(cumparaturi):
    produsul=input("Introdu denumirea produsului: ").lower()
    if produsul in cumparaturi:
        print("Produsul deja este prezent in lista")
    else:
        cumparaturi.append(produsul)
        print(produsul.title() + " a fost adaugat in lista")

def sterge(cumparaturi):
    alegere=int(input("Cum vrei sa stergi produsul? 2- dupa numarul in lista; 1- dupa denumirea produsului: "))
    if alegere == 1:
        produsul = input("Introduceti denumirea produsului care trebuie sters din lista: ").lower()
        if produsul not in cumparaturi:
            print("Nu este un produs cu aceasta denumire in lista\n")
        else:
            cumparaturi.remove(produsul)
            print("Produsul a fost eliminat din lista.\n")
    elif alegere == 2:
        pozitia = int(input("Introduceti pozitia produsului care trebuie sters: "))
        if pozitia <= 0 or pozitia > len(cumparaturi):
            print("Pozitie invalida. Te rog sa introduci un numar valid.\n")
        else:
            del cumparaturi[pozitia-1]
            print("Produsul a fost eliminat din lista.\n")
    else:
        print("Optiune invalida! Te rog sa alegi 1 sau 2.\n")

def afisare(cumparaturi):
    if not cumparaturi:
            print("Nu sunt asa produse in lista")
    else:
        for index, produsul in enumerate(cumparaturi):
            print(index+1 ,") ", produsul)