//import java.util.*;
//import java.util.stream.Collectors;
//
//public class Test {
//    public static void main(String[] args) {
//        TreeMap<String, Prsn> map = new TreeMap<>();
//        TreeSet<Prsn> set = new TreeSet<>();
//        HashSet<Prsn> hashSet = new HashSet<>();
//        LinkedList<Prsn> list = new LinkedList<>();
//
//        Prsn prsn0 = new Prsn("a", 1, 1);
//        Prsn prsn1 = new Prsn("b", 2, 2);
//        Prsn prsn2 = new Prsn("c", 3, 3);
//        Prsn prsn3 = new Prsn("c", 3, 3);
//        Prsn prsn4 = new Prsn("c", 3, 3);
//        Prsn prsn5 = new Prsn("c", 3, 3);
//        Prsn prsn6 = new Prsn("c", 3, 3);
//
//
//        // TreeMap sorted map -> sorted by Key(String)
////        map.put(prsn0.getName(), prsn0);
////        map.put(prsn0.getName(), prsn0);
////        map.put(prsn1.getName(), prsn1);
//
//        //TreeSet sorted no duplicates - implements Comparable needed
////        set.add(prsn0);
////        set.add(prsn0);
////        set.add(prsn1);
////        set.add(prsn2);
////        set.add(prsn3);
////        set.add(prsn4);
////        set.add(prsn5);
////        set.add(prsn6);
////
////        TreeSet<Prsn> set2 = set.stream().filter(prsn -> prsn.getName().equals("c")).collect(Collectors.toCollection(TreeSet<Prsn>::new));
////        // HashSet no duplicates
////        hashSet.add(prsn0);
////        hashSet.add(prsn0);
////        hashSet.add(prsn1);
////        hashSet.add(prsn2);
////        hashSet.add(prsn3);
//
////        //LinkedList
////        list.add(prsn0);
////        list.add(prsn1);
////        list.add(prsn2);
////        list.add(prsn3);
////        list.add(prsn4);
////        list.add(prsn5);
////        list.add(prsn6);
////
////        int index = list.indexOf(prsn0);
////        System.out.println(index);
////        Iterator<Prsn> iter  = list.iterator();
////        while (iter.hasNext()){
////            System.out.println(iter.next());
////        }
//
//
//
//
//
//
//    }
//
//}
//
//class Prsn  implements Comparable{
//    private String name;
//    private int age;
//    private int money;
//
//    public Prsn(String name, int age, int money) {
//        this.name = name;
//        this.age = age;
//        this.money = money;
//    }
//
//    public String getName() {
//        return name;
//    }
//
//    public int getAge() {
//        return age;
//    }
//
//    public int getMoney() {
//        return money;
//    }
//
//    @Override
//    public boolean equals(Object o) {
//        if (o == null || getClass() != o.getClass()) return false;
//        Prsn prsn = (Prsn) o;
//        return age == prsn.age;
//    }
//
//    @Override
//    public int hashCode() {
//        return Objects.hash(name, age, money);
//    }
//
//    @Override
//    public String toString() {
//        return "Prsn{" +
//                "name='" + name + '\'' +
//                ", age=" + age +
//                ", money=" + money +
//                '}' + "\n";
//    }
//
//    @Override
//    public int compareTo(Object o) {
//        return this.name.compareTo(((Prsn)o).getName());
//    }
//}
