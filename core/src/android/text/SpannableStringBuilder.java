package android.text;
public class SpannableStringBuilder implements Editable, Spannable {
    private final StringBuilder sb;
    public SpannableStringBuilder() { sb = new StringBuilder(); }
    public SpannableStringBuilder(CharSequence s) { sb = new StringBuilder(s == null ? "" : s.toString()); }
    public int length() { return sb.length(); }
    public char charAt(int i) { return sb.charAt(i); }
    public CharSequence subSequence(int s, int e) { return new SpannableStringBuilder(sb.substring(s, e)); }
    public String toString() { return sb.toString(); }
    public SpannableStringBuilder replace(int st, int en, CharSequence t) { sb.replace(st, en, t.toString()); return this; }
    public SpannableStringBuilder insert(int w, CharSequence t) { sb.insert(w, t.toString()); return this; }
    public SpannableStringBuilder delete(int st, int en) { sb.delete(st, en); return this; }
    public SpannableStringBuilder append(CharSequence t) { sb.append(t); return this; }
    public SpannableStringBuilder append(CharSequence t, int s, int e) { sb.append(t, s, e); return this; }
    public SpannableStringBuilder append(char c) { sb.append(c); return this; }
    public SpannableStringBuilder append(CharSequence t, Object what, int flags) { sb.append(t); return this; }
    public void clear() { sb.setLength(0); }
    public void clearSpans() {}
    public void setSpan(Object what, int s, int e, int f) {}
    public void removeSpan(Object what) {}
    public boolean equals(Object o) { return o instanceof CharSequence && toString().equals(o.toString()); }
    public int hashCode() { return toString().hashCode(); }
}
