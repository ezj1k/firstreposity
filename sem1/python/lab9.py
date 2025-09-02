import datetime
import calendar
import re
import math

#SARCINA1

# def introdu_date():
#     while True:
#         # "do" — cere introducerea datei
#         data_input = input("Introduceti data nasterii (format YYYY-MM-DD): ")

#         # Verifica formatul cu expresie regulata
#         if not re.match(r"^\d{4}-\d{2}-\d{2}$", data_input):
#             print("Format invalid! Folositi formatul YYYY-MM-DD.")
#             continue  # reia introducerea

#         # Incearca sa extraga anul, luna si ziua
#         try:
#             an, luna, zi = map(int, data_input.split('-'))
#         except ValueError:
#             print("Datele trebuie sa fie numere intregi.")
#             continue

#         # Verificare limita luna
#         if not (1 <= luna <= 12):
#             print("Luna trebuie sa fie intre 1 si 12.")
#             continue

#         # Verificare limita zi (in functie de luna)
#         zile_in_luna = calendar.monthrange(an, luna)[1]
#         if not (1 <= zi <= zile_in_luna):
#             print(f"Ziua trebuie sa fie intre 1 si {zile_in_luna} pentru luna {luna}.")
#             continue

#         # Verificare data in viitor
#         data_nasterii = datetime.date(an, luna, zi)
#         data_curenta = datetime.date.today()

#         if data_nasterii > data_curenta:
#             print("Data de nastere nu poate fi in viitor.")
#             continue

#         # Daca toate verificarile au trecut, iesim din bucla
#         break

#     # Calculam si afisam diferenta in zile
#     diferenta = data_curenta - data_nasterii
#     print(f"Ati trait {diferenta.days} zile pana astazi.")

# # Apelarea functiei
# introdu_date()

# #SARCINA2

# def zi_saptamana():
#     zile_saptamana = ["Luni", "Marti", "Miercuri", "Joi", "Vineri", "Sambata", "Duminica"]

#     while True:
#         data_input = input("Introduceti o data (format YYYY-MM-DD): ")

#         # Verificare format cu expresie regulata
#         if not re.match(r"^\d{4}-\d{2}-\d{2}$", data_input):
#             print("Format invalid! Folositi formatul YYYY-MM-DD.")
#             continue

#         try:
#             an, luna, zi = map(int, data_input.split('-'))
#             datetime.date(an, luna, zi)  # Verifica daca data este valida
#         except ValueError:
#             print("Data introdusa nu este valida.")
#             continue

#         # Determina ziua saptamanii (0 = Luni, 6 = Duminica)
#         zi_index = calendar.weekday(an, luna, zi)
#         zi_text = zile_saptamana[zi_index]

#         print(f"Data {data_input} este o zi de {zi_text}.")
#         break

# # Apelam functia
# zi_saptamana()

# #SARCINA3

# def calculeaza_timp_cadere():
#     g = 9.8  # acceleratia gravitationala in m/s^2

#     while True:
#         h_input = input("Introduceti inaltimea de la care cade obiectul (in metri): ")

#         try:
#             h = float(h_input)

#             # Verificam daca valoarea este NaN (Not a Number)
#             if math.isnan(h):
#                 print("Valoarea introdusa nu este un numar valid (NaN).")
#                 continue

#             if h <= 0:
#                 print("Inaltimea nu poate fi negativa sau 0.")
#                 continue

#             break  # iesim din bucla daca totul e valid

#         except ValueError:
#             print("Valoarea introdusa nu este un numar.")
#             continue

#     # Calculam timpul de cadere
#     t = math.sqrt((2 * h) / g)
#     print(f"Timpul de cadere este aproximativ {t:.2f} secunde.")

# # Apelam functia
# calculeaza_timp_cadere()
# #SARCINA4

def afiseaza_duminici():
    while True:
        try:
            an = int(input("Introdu anul (ex: 2025): "))
            luna = int(input("Introdu luna (1-12): "))
            
            if not (1 <= luna <= 12):
                print("Luna trebuie să fie între 1 și 12.")
                continue

            break  # ieșim din buclă dacă totul e valid
        except ValueError:
            print("Te rog introdu doar numere întregi pentru an și lună.")

    # Aflăm câte zile are luna dată
    zile_in_luna = calendar.monthrange(an, luna)[1] # pentru iunie 2025/ (6, 30) -- 6 = prima zi a lunii iunie 2025 este duminică -- 30 = iunie are 30 de zile

    print(f"\nDuminicile din {calendar.month_name[luna]} {an} sunt:\n")

    for zi in range(1, zile_in_luna + 1):
        data = datetime.date(an, luna, zi)
        if data.weekday() == 6:  # 6 = duminică
            print(data.strftime("%A, %d %B %Y")) #Sunday, 15 June 2025

# Apelăm funcția
afiseaza_duminici()
