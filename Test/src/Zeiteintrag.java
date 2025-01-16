
public class Zeiteintrag{
    private String phase;
    private String datum;
    private String zeitVon;
    private String zeitBis;

    public String getDatum() {
        return datum;
    }

    public Zeiteintrag(String phase, String datum, String zeitVon, String zeitBis) {
        this.phase = phase;
        this.datum = datum;
        this.zeitVon = zeitVon;
        this.zeitBis = zeitBis;
    }

    public String getPhase() {
        return phase;
    }

    public String getZeitVon() {
        return zeitVon;
    }

    public String getZeitBis() {
        return zeitBis;
    }

}
