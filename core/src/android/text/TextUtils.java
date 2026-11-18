package android.text;
public class TextUtils {
    public enum TruncateAt { START, MIDDLE, END, MARQUEE }
    public static boolean isEmpty(CharSequence s) { return s == null || s.length() == 0; }
    public static boolean equals(CharSequence a, CharSequence b) {
        if (a == b) return true;
        if (a == null || b == null || a.length() != b.length()) return false;
        return a.toString().equals(b.toString());
    }
    public static String join(CharSequence d, Iterable<?> parts) {
        StringBuilder sb = new StringBuilder(); boolean f = true;
        for (Object p : parts) { if (!f) sb.append(d); sb.append(p); f = false; }
        return sb.toString();
    }
    public static String join(CharSequence d, Object[] parts) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < parts.length; i++) { if (i > 0) sb.append(d); sb.append(parts[i]); }
        return sb.toString();
    }
    public static String[] split(String s, String re) { return s.length() == 0 ? new String[0] : s.split(re, -1); }
    public static boolean isDigitsOnly(CharSequence s) { for (int i = 0; i < s.length(); i++) if (!Character.isDigit(s.charAt(i))) return false; return true; }
    public static int getTrimmedLength(CharSequence s) { return s.toString().trim().length(); }
    public static CharSequence concat(CharSequence... t) { StringBuilder sb = new StringBuilder(); for (CharSequence c : t) sb.append(c); return sb.toString(); }
    public static String htmlEncode(String s) { return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;"); }
}
