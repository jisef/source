import java.util.ArrayList;
import java.util.Comparator;

public class Person {
    private String name;
    // muss sortiert sein
    private ArrayList<Zeiteintrag> zeiteintrag;

    public Person(String name) {
        this.name = name;
        this.zeiteintrag = new ArrayList<Zeiteintrag>();
    }

    public Person(String name, Zeiteintrag zeiteintrag) {
        this.name = name;
        this.zeiteintrag = new ArrayList<Zeiteintrag>();
        this.zeiteintrag.add(zeiteintrag);
    }

    public void addZeiteintrag(Zeiteintrag zeiteintrag){
        this.zeiteintrag.add(zeiteintrag);
        this.zeiteintrag.sort(Comparator.comparing(Zeiteintrag::getZeitVon)
                .thenComparing(Zeiteintrag::getZeitVon)
                .thenComparing(Zeiteintrag::getZeitBis)
        );
    }

    public ArrayList<Zeiteintrag> getZeiteintrag() {
        return zeiteintrag;
    }

    public String getName() {
        return name;
    }
}
