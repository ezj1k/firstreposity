# #sarcina 1
# greutate=int(input("Greutate(kg)(30-300):" ))
# inaltime=int(input("inaltime(cm)(150-220): " ))
# varsta=int(input("varsta(ani)(20-120): " ))
# sex=input("sex (M/F): " )
# while (sex!="M" and sex!="F"):
#     sex=input("sex (M/F): " )
# while greutate<30 or greutate>300:
#     greutate=input("Greutate(kg):" )
# while inaltime<150 or inaltime>220:
#     inaltime=input("inaltime(cm): " )
# while varsta<20 or varsta>120:
#     varsta=input("varsta(ani): " )
    
# if (sex=="M"):
#     Greutatea_ideala = inaltime - 100 - ((inaltime - 150)/4 + (varsta - 20)/4)
# elif (sex=="F"):
#     Greutatea_ideala = inaltime - 100 - ((inaltime - 150)/2.5 + (varsta - 20)/6)

# if greutate-Greutatea_ideala>0:
#     print("greutatea este mai mare decat cea asteptata")
# elif greutate-Greutatea_ideala<0:
#     print("greutatea este mai mica decat cea asteptata")

#sarcina 2
def numar(luni):
    corespondenta={
        "pisica": "om",
        "1": "6 luni",
        "2": "10 luni",
        "3": "2 ani",
        "4": "5 ani",
        "5": "8 ani",
        "6": "14 ani",
        "7": "15 ani",
        "8": "16 ani",
        "10": "17 ani",
        "11": "17 ani",
    }
    print('pisica are ',corespondenta[str(luni)])

raspuns=input("pisica este mai mică decât un an? (Da/Nu sau Yes/No)." )
while (raspuns!="Da" and raspuns!="Yes" and raspuns!="Nu" and raspuns!="No"):
    raspuns=input("Da/Nu or Yes/No)." )
if (raspuns=="Da" or raspuns=="Yes"):
    luni=int(input("cate luni are pisicul? (1-11): "))
    while luni>11 and luni<1 and luni!=9:
        luni=int(input("1-11: "))

    numar(luni)
    
#sarcina 3
elif (raspuns=="Nu" or raspuns=="No"):
    x=25
    y=77
    ani=int(input("cati ani are pisicul? (1-35): "))
    while ani>35 and ani<1:
        ani=int(input("1-35: "))
    if ani==1:
        print("În ani omenești pisicul tău ar avea 18 ani")
    elif ani==2:
        print("În ani omenești pisicul tău ar avea 25 ani")
    elif ani>=16 and ani<=35:
        for i in range(3,ani):
            x+=4
            print("În ani omenești pisicul tău ar avea"+x+"ani")
    elif ani>=3 and ani<=15:
        for i in range(3,ani):
            y+=3
            print("În ani omenești pisicul tău ar avea"+y+"ani")