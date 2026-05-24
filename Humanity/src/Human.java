import java.util.Random;
abstract class Human implements Celebration {
    String name;
    int age;
    static int maxLimit = 200;

    public Human() {
        String names[] = {"Ion", "Alex", "Sasha", "Paul", "Steve", "Ann"};
        name = names [new Random().nextInt(names.length)];
        age = new Random().nextInt(maxLimit);
    }

    public Human(String a, int b) {
        name = a;
        if(b<maxLimit && b>0) {
            age = b;
        }
    }

    public void println() {
        System.out.println("My name is" + name+", my years is "+age);
    }

    abstract void sayHi();
}
