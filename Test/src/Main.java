public class Main {
    public static void main(String[] args) {
        String[] s = "123;123;".split(";");
        for (int i = 0; i < s.length; i++) {
            System.out.println(s[i]);
        }
    }
}