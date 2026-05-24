public class ColsBestPrice implements Runnable {
    int id;

    ColsBestPrice(int x) {
        id = x;
    }

    public void run() {
        Matura lowest = Main.raft[0][id];

        for (int i = 1; i<Main.rows; i++) {
            if(lowest.maturaPrice > Main.raft[i][id].maturaPrice) {
                lowest = Main.raft[i][id];
            }
        }

        Main.theLowestPrice[id] = lowest;
    }
}
