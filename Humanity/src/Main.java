public class Main {
    public static void main(String[] args) {
        Human[] party = new Human[6];
        for(int i =0; i<party.length;i+=2) {
            party[i] = new Man();
            party[i+1] = new Woman();
        }
        for(int i =0; i<party.length;i++) {
            party[i].println();
        }

        Human Vasea = new Man();
        Vasea.println();
        ((Man)Vasea).drive();
        //de gasit toate femeile la petrecere si facut machiaj

        for(int i=0; i<party.length; i++) {
            if (party[i] instanceof Woman) {
                ((Woman)party[i]).makeUp();
            }
        }

        for(int i=0; i<party.length; i++) {
            if (party[i] instanceof Man) {
                ((Man)party[i]).drive();
            }
        }

        for (int i = 0; i< party.length; i++) {
            party[i].sayHi();
            party[i].WineDay();
        }
        for (int i = 0; i< party.length; i++) {
            party[i].sayHi();
        }
    }
}
