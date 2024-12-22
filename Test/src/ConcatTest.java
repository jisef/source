public class ConcatTest{
    private static final int CONST = 900000;
    public static String concatWithString()    {
        String t = "Java";
        for (int i=0; i < CONST; i++){
            System.out.println(i);
            t = t + "Tpoint";

        }
        return t;
    }
    public static String concatWithStringBuffer(){
        StringBuffer sb = new StringBuffer("Java");
        for (int i=0; i < CONST; i++){
            sb.append("Tpoint");
        }
        return sb.toString();
    }
    public static void main(String[] args){
        for (int i = 0; i < 1; i++) {
            long startTime = System.currentTimeMillis();
            concatWithString();
            System.out.println("Time taken by Concating with String: "+(System.currentTimeMillis()-startTime)+"ms");
            startTime = System.currentTimeMillis();
            concatWithStringBuffer();
            System.out.println("Time taken by Concating with  StringBuffer: "+(System.currentTimeMillis()-startTime)+"ms");

        }
    }
}