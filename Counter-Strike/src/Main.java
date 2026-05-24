public class Main {
    public static void main(String[] args) {
        Player[] players = new Player[4];
        players[0] = new Terrorist();
        players[1] = new CounterTerrorist();
        players[2] = new Bomber();
        players[3] = new Defuser();

        System.out.println("PLAYER DETAILS:");
        for (int i = 0; i < players.length; i++) {
            players[i].println();
            players[i].shooting();
            System.out.println();
        }

        for (int i = 0; i < players.length; i++) {
            if (players[i] instanceof Terrorist) {
                Terrorist terrorist = (Terrorist) players[i];
                terrorist.changeGun("AWP");

                if (players[i] instanceof Bomber) {
                    ((Bomber) players[i]).detonate();
                }
            }

            if (players[i] instanceof CounterTerrorist) {
                CounterTerrorist counterTerrorist = (CounterTerrorist) players[i];
                counterTerrorist.buyNewArmor();

                if (players[i] instanceof Defuser) {
                    ((Defuser) players[i]).detonate();
                }
            }
        }

        System.out.println();

        System.out.println("BOMB INTERACTION:");
        BombPlanted[] bombHandlers = new BombPlanted[2];
        bombHandlers[0] = (Terrorist) players[0];
        bombHandlers[1] = (Defuser) players[3];

        for (BombPlanted handler : bombHandlers) {
            handler.interactionBomb();
        }



    }
}
