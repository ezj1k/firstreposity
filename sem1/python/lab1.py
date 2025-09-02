Salut="salut"
print(Salut)
name=(input("introdu numele tau"))
com="""poti sal introduci ca string"""
#comentariu
v1=10
v2=3.12
v3="text"
v4="""text
in
3-4
randuri"""
print(type(v1))
print(len(v3))
print(v3.upper())
print(v3[1:3])
mesaj="scrie te rog un {}".format(v3)
print(mesaj)

txt = "More re sults from text..."
substr = txt[4:12]
print(substr)
input()
print(substr.strip(),'\n') 

txt = "More results from text..."
print(txt.split(),"\n")

age = 36
txt = "My name is Mary, and I am {}"
print(txt.format(age))
txt=txt.replace(" ", "")
print(txt.format(age))
