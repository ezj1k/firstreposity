import re
from functii import Employee, HourlyEmployee, SalaryEmployee

def valid_input(prompt, pattern, error_msg):
    while True:
        val = input(prompt)
        if re.fullmatch(pattern, val):
            return val
        else:
            print(error_msg)

def create_employee(cls_type):
    obj = cls_type()

    obj.nume = valid_input("Nume: ", r"[A-Za-z]+", "Numele trebuie să conțină doar litere.")
    obj.telefon = valid_input("Telefon (+373xxxxxxxx): ", r"\+373\d{8}", "Telefon invalid.")
    obj.data_nasterii = valid_input("Data nașterii (zz.ll.aaaa): ",
                                    r"(0[1-9]|[12][0-9]|3[01])\.(0[1-9]|1[012])\.(19[6-9][0-9]|200[0-7])",
                                    "Dată invalidă.")
    obj.email = valid_input("Email: ", r"[A-Za-z0-9_.-]{2,20}@[A-Za-z]{4,7}\.[A-Za-z]{2,4}", "Email invalid.")
    obj.specialitate = valid_input("Specialitate: ", r"[A-Za-z]{4,20}", "Specialitate invalidă.")

    if isinstance(obj, HourlyEmployee):
        obj.ore_lucrate = int(valid_input("Ore lucrate: ", r"\d+", "Număr invalid."))
        obj.plata_per_ora = float(valid_input("Plată per oră: ", r"\d+(\.\d+)?", "Valoare invalidă."))
    elif isinstance(obj, SalaryEmployee):
        obj.salariu_fix = float(valid_input("Salariu fix: ", r"\d+(\.\d+)?", "Valoare invalidă."))

    return obj

# Creăm obiectele
employees = []
print("Introduceți 2 angajați de tip 'Employee':")
for _ in range(2):
    employees.append(create_employee(Employee))

print("\nIntroduceți 2 angajați de tip 'HourlyEmployee':")
for _ in range(2):
    employees.append(create_employee(HourlyEmployee))

print("\nIntroduceți 2 angajați de tip 'SalaryEmployee':")
for _ in range(2):
    employees.append(create_employee(SalaryEmployee))

# Afișare informații
print("\nInformațiile angajaților:")
for e in employees:
    print("\n-------------------")
    print(f"Nume: {e.nume}")
    print(f"Telefon: {e.telefon}")
    print(f"Data nașterii: {e.data_nasterii} (vârsta: {e.calculareVarsta()} ani)")
    print(f"Email: {e.email}")
    print(f"Specialitate: {e.specialitate}")
    if isinstance(e, HourlyEmployee):
        print(f"Ore lucrate: {e.ore_lucrate}, Plata/oră: {e.plata_per_ora}")
    if isinstance(e, SalaryEmployee):
        print(f"Salariu fix: {e.salariu_fix}")

# Afișare salarii
salarii_hourly = [e._calculareSalariu() for e in employees if isinstance(e, HourlyEmployee)]
salarii_salary = [e._calculareSalariu() for e in employees if isinstance(e, SalaryEmployee)]

print("\nSalarii HourlyEmployee:", salarii_hourly)
print("Salarii SalaryEmployee:", salarii_salary)
