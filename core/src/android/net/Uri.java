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
    public Builder buildUpon(){String s=toString();Builder b=new Builder();int hash=s.indexOf('#');if(hash>=0){b.fragment=s.substring(hash+1);s=s.substring(0,hash);}int q=s.indexOf('?');if(q>=0){b.query=s.substring(q+1);s=s.substring(0,q);}int scheme=s.indexOf(':');if(scheme>=0){b.scheme=s.substring(0,scheme);s=s.substring(scheme+1);}if(s.startsWith("//")){s=s.substring(2);int slash=s.indexOf('/');b.authority=slash<0?s:s.substring(0,slash);s=slash<0?"":s.substring(slash);}b.path=s;return b;}
    public static final class Builder { private String scheme,authority,path="",query,fragment; public Builder scheme(String s){scheme=s;return this;} public Builder authority(String s){authority=s;return this;} public Builder encodedAuthority(String s){return authority(s);} public Builder path(String s){path=s;return this;} public Builder encodedPath(String s){return path(s);} public Builder appendPath(String s){return appendEncodedPath(Uri.encode(s));} public Builder appendEncodedPath(String s){if(path==null)path="";if(!path.endsWith("/"))path+="/";path+=s;return this;} public Builder query(String q){query=q;return this;} public Builder encodedQuery(String q){return query(q);} public Builder appendQueryParameter(String key,String value){String part=Uri.encode(key)+"="+Uri.encode(value);query=query==null?part:query+"&"+part;return this;} public Builder clearQuery(){query=null;return this;} public Builder fragment(String f){fragment=f;return this;} public Builder encodedFragment(String f){return fragment(f);} public Uri build(){StringBuilder b=new StringBuilder();if(scheme!=null)b.append(scheme).append(':');if(authority!=null)b.append("//").append(authority);if(path!=null)b.append(path);if(query!=null)b.append('?').append(query);if(fragment!=null)b.append('#').append(fragment);return Uri.parse(b.toString());} }
    public int compareTo(Uri o) { return toString().compareTo(o.toString()); }
    public boolean equals(Object o) { return o instanceof Uri && toString().equals(o.toString()); }
    public int hashCode() { return toString().hashCode(); }
}
