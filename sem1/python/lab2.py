list0 = [1, 2, 3, "str", True]
print(list0[0])
print(list0[2])
list0[2]=10
print(list0)
print(list0[1:3])
print(type(list0))
print(len(list0))
print(list0[3].split())
print(list0[0+1])
print(list0[0]+1)
print(list0[2]/2)
tuplu=(1, 2, 3, 4, 5, "str str")
print(type(tuplu))
print(tuplu[0], len(tuplu))
print(tuplu[1:3])
tuplu1=(3,4,5)
print(len(tuplu), tuplu1+tuplu, tuplu.index(2) )

set1={1,2,2,2,3,4,5}
print(set1)
set1.add(7)
print(type(set1), set1)
dict1={"key1":1, "key2":2, "key3":3}
dict2={10:1, 11:"str", 12:True}
print(dict1("key1"), dict2(11))
dict1["key4"]=4
dict1["key2"]=10
print(dict1)
print(len(dict1))
print(type(dict2))
listNou=list(tuplu)
listNr=[1,2,3]
listTxt=["mere", "pere", "nuci"]
info="{} coasta {} \n {} coasta {} \n {} coasta {} \n"
print(info.format(listTxt[0], listNr[0], listTxt[1], listNr[1], listTxt[2], listNr[2]))

print("introdu varsta")
x=input()
int(x)
x=x+5
text="in 5 ani vei avea {}"
print(text.format(x))

if 3 in list0:
    print("este")
else:
    print("nu este")