import datetime

class Employee:
    def __init__(self):
        self.__nume = None
        self.__telefon = None
        self.__data_nasterii = None
        self.__email = None
        self.__specialitate = None

    # Getter și Setter clasic cu property()
    def get_nume(self):
        return self.__nume

    def set_nume(self, nume):
        self.__nume = nume

    nume = property(get_nume, set_nume)

    # Getter și setter cu decoratori
    @property
    def telefon(self):
        return self.__telefon

    @telefon.setter
    def telefon(self, value):
        self.__telefon = value

    @property
    def data_nasterii(self):
        return self.__data_nasterii

    @data_nasterii.setter
    def data_nasterii(self, value):
        self.__data_nasterii = value

    @property
    def email(self):
        return self.__email

    @email.setter
    def email(self, value):
        self.__email = value

    @property
    def specialitate(self):
        return self.__specialitate

    @specialitate.setter
    def specialitate(self, value):
        self.__specialitate = value

    def calculareVarsta(self):
        if self.__data_nasterii:
            zi, luna, an = map(int, self.__data_nasterii.split('.'))
            data_nasterii = datetime.date(an, luna, zi)
            azi = datetime.date.today()
            return azi.year - data_nasterii.year - ((azi.month, azi.day) < (data_nasterii.month, data_nasterii.day))
        return None

    def _calculareSalariu(self):
        pass


class HourlyEmployee(Employee):
    def __init__(self):
        super().__init__()
        self.__ore_lucrate = 0
        self.__plata_per_ora = 0.0

    @property
    def ore_lucrate(self):
        return self.__ore_lucrate

    @ore_lucrate.setter
    def ore_lucrate(self, value):
        self.__ore_lucrate = value

    @property
    def plata_per_ora(self):
        return self.__plata_per_ora

    @plata_per_ora.setter
    def plata_per_ora(self, value):
        self.__plata_per_ora = value

    def _calculareSalariu(self):
        return self.__ore_lucrate * self.__plata_per_ora


class SalaryEmployee(Employee):
    def __init__(self):
        super().__init__()
        self.__salariu_fix = 0.0

    @property
    def salariu_fix(self):
        return self.__salariu_fix

    @salariu_fix.setter
    def salariu_fix(self, value):
        self.__salariu_fix = value

    def _calculareSalariu(self):
        return self.__salariu_fix
