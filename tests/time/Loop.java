public class Loop {
    public static void main(String[] args) {
        long i = 0;
        long acc = 0;
        while (i < 1000000) {
            acc = (acc + i * 3) % 1000007;
            i = i + 1;
        }
        System.out.println(acc);
    }
}