package android.text;
public class SpannableString implements Spannable {
    private final String s;
    public SpannableString(CharSequence s) { this.s = s.toString(); }
    public static SpannableString valueOf(CharSequence s) { return new SpannableString(s); }
    public int length() { return s.length(); }
    public char charAt(int i) { return s.charAt(i); }
    public CharSequence subSequence(int a, int b) { return s.subSequence(a, b); }
    public String toString() { return s; }
    public void setSpan(Object what, int a, int b, int f) {}
    public void removeSpan(Object what) {}
}
