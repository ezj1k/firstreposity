import re

# Realizati un program care in ciclu va testa lista 'test' cu un pattern regex si va crea o alta lista cu valori de True si False, patter-nul valideaza 3 caractere(a,b sau c ele pot sa se repete) si un numar de la 10 la 20.  
test = ['abc10','bca09','cab20','abc30','cab18','abc00','cab11','aad19']
rezultat = [ True,False,True,False,True,False,True,False ] 
rezultat2=[]

regextest=r"^[a-c]{3}(10|1[1-9]|20)$"
for item in test:
    x = re.search(regextest, item)
    rezultat2.append(bool(x))

da=True

for i in range(len(rezultat)):
    if rezultat[i]!=rezultat2[i]:
        print("nu sunt la fel")
        da=False
        break

if da==True:
    print("tot corect")