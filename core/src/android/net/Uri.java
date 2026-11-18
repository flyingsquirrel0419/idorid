package android.net;
public abstract class Uri implements Comparable<Uri> {
    public static final Uri EMPTY = parse("");
    public static Uri parse(final String s) {
        return new Uri() { public String toString() { return s; } };
    }
    public static Uri fromParts(String scheme, String ssp, String frag) { return parse(scheme + ":" + ssp + (frag == null ? "" : "#" + frag)); }
    public static String encode(String s) { return s; }
    public static String decode(String s) { return s; }
    public String getScheme() { String s = toString(); int i = s.indexOf(':'); return i < 0 ? null : s.substring(0, i); }
    public String getHost() {
        String s = toString(); int i = s.indexOf("://"); if (i < 0) return null;
        String rest = s.substring(i + 3); int j = rest.indexOf('/'); return j < 0 ? rest : rest.substring(0, j);
    }
    public String getPath() {
        String s = toString(); int i = s.indexOf("://"); String rest = i < 0 ? s : s.substring(i + 3);
        int j = rest.indexOf('/'); if (j < 0) return ""; String p = rest.substring(j); int q = p.indexOf('?'); return q < 0 ? p : p.substring(0, q);
    }
    public String getLastPathSegment() { String p = getPath(); int i = p.lastIndexOf('/'); return p.substring(i + 1); }
    public String getQueryParameter(String key) {
        String s = toString(); int q = s.indexOf('?'); if (q < 0) return null;
        for (String kv : s.substring(q + 1).split("&")) { int e = kv.indexOf('='); if (e > 0 && kv.substring(0, e).equals(key)) return kv.substring(e + 1); }
        return null;
    }
    public int compareTo(Uri o) { return toString().compareTo(o.toString()); }
    public boolean equals(Object o) { return o instanceof Uri && toString().equals(o.toString()); }
    public int hashCode() { return toString().hashCode(); }
}
