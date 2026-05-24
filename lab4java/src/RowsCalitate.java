public class RowsCalitate extends Thread {
    int id;

    RowsCalitate(int x) {
        id = x;
    }

    public void run() {
        Matura best = Main.raft[id][0];

        for (int i = 1; i<Main.cols; i++) {
            if(best.maturaCalitate < Main.raft[id][i].maturaCalitate) {
                best = Main.raft[id][i];
            }
        }

        Main.bestOfTheBest[id] = best;
    }
}
