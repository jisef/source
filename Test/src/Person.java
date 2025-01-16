import java.util.ArrayList;
import java.util.Comparator;
import java.util.TreeSet;

public class Person {
    private String name;
    // TODO: has to be sorted
    private TreeSet<Zeiteintrag> zeiteintrag;

    public Person(String name) {
        this.name = name;
        this.zeiteintrag = new TreeSet<Zeiteintrag>();
    }

    public Person(String name, Zeiteintrag zeiteintrag) {
        this.name = name;
        this.zeiteintrag = new TreeSet<Zeiteintrag>();
        this.zeiteintrag.add(zeiteintrag);
    }

    public void addZeiteintrag(Zeiteintrag zeiteintrag){
        this.zeiteintrag.add(zeiteintrag);
    }

    public TreeSet<Zeiteintrag> getZeiteintrag() {
        return zeiteintrag;
    }

    public String getName() {
        return name;
    }
}
