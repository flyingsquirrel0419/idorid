package java.lang;

import java.util.Comparator;
import java.util.Locale;

/** Backed by a VM-native compact string (Latin-1 or UTF-16). No Java fields. */
public final class String implements java.io.Serializable, Comparable<String>, CharSequence {
    public static final Comparator<String> CASE_INSENSITIVE_ORDER = new Comparator<String>() {
        public int compare(String a, String b) { return a.compareToIgnoreCase(b); }
    };

    public String() {}
    public String(String s) { initChars(s.toCharArray(), 0, s.length()); }
    public String(char[] v) { initChars(v, 0, v.length); }
    public String(char[] v, int off, int len) { initChars(v, off, len); }
    public String(byte[] b) { initBytes(b, 0, b.length); }
    public String(byte[] b, int off, int len) { initBytes(b, off, len); }
    public String(byte[] b, String charset) { initBytes(b, 0, b.length); }
    public String(byte[] b, int off, int len, String charset) { initBytes(b, off, len); }
    public String(byte[] b, java.nio.charset.Charset cs) { initBytes(b, 0, b.length); }
    public String(StringBuilder sb) { initChars(sb.getValue(), 0, sb.length()); }
    public String(StringBuffer sb) { this(sb.toString()); }
    public String(int[] cps, int off, int count) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < count; i++) sb.appendCodePoint(cps[off + i]);
        initChars(sb.getValue(), 0, sb.length());
    }

    private native void initChars(char[] v, int off, int len);
    private native void initBytes(byte[] b, int off, int len);

    public native int length();
    public boolean isEmpty() { return length() == 0; }
    public native char charAt(int i);
    public int codePointAt(int i) { return Character.codePointAt(this, i); }
    public native boolean equals(Object o);
    public native int hashCode();
    public native int compareTo(String o);
    public int compareToIgnoreCase(String o) { return toLowerCase().compareTo(o.toLowerCase()); }
    public native boolean equalsIgnoreCase(String o);
    public boolean contentEquals(CharSequence cs) { return equals(cs.toString()); }

    public int indexOf(int ch) { return indexOf(ch, 0); }
    public native int indexOf(int ch, int from);
    public int indexOf(String s) { return indexOf(s, 0); }
    public native int indexOf(String s, int from);
    public int lastIndexOf(int ch) { return lastIndexOf(ch, length() - 1); }
    public native int lastIndexOf(int ch, int from);
    public int lastIndexOf(String s) { return lastIndexOf(s, length()); }
    public native int lastIndexOf(String s, int from);
    public boolean contains(CharSequence s) { return indexOf(s.toString()) >= 0; }

    public String substring(int b) { return substring(b, length()); }
    public native String substring(int b, int e);
    public CharSequence subSequence(int b, int e) { return substring(b, e); }
    public native String concat(String s);
    public native String intern();
    public native char[] toCharArray();
    public native void getChars(int sb, int se, char[] dst, int db);
    public native byte[] getBytes();
    public byte[] getBytes(String cs) { return getBytes(); }
    public byte[] getBytes(java.nio.charset.Charset cs) { return getBytes(); }
    public native boolean startsWith(String p, int off);
    public boolean startsWith(String p) { return startsWith(p, 0); }
    public boolean endsWith(String s) { return startsWith(s, length() - s.length()); }
    public native String toLowerCase();
    public native String toUpperCase();
    public String toLowerCase(Locale l) { return toLowerCase(); }
    public String toUpperCase(Locale l) { return toUpperCase(); }
    public native String trim();
    public String strip() { return trim(); }
    public boolean isBlank() { return trim().isEmpty(); }
    public native String replace(char a, char b);
    public String replace(CharSequence target, CharSequence repl) {
        String t = target.toString(), r = repl.toString();
        if (t.isEmpty()) {
            StringBuilder sb = new StringBuilder(r);
            for (int i = 0; i < length(); i++) { sb.append(charAt(i)); sb.append(r); }
            return sb.toString();
        }
        int i = indexOf(t);
        if (i < 0) return this;
        StringBuilder sb = new StringBuilder();
        int last = 0;
        while (i >= 0) {
            sb.append(this, last, i).append(r);
            last = i + t.length();
            i = indexOf(t, last);
        }
        sb.append(this, last, length());
        return sb.toString();
    }
    public native boolean matches(String regex);
    public native String replaceAll(String regex, String repl);
    public native String replaceFirst(String regex, String repl);
    public String[] split(String regex) { return split(regex, 0); }
    public native String[] split(String regex, int limit);
    public boolean regionMatches(int to, String other, int oo, int len) { return regionMatches(false, to, other, oo, len); }
    public boolean regionMatches(boolean ic, int to, String other, int oo, int len) {
        if (to < 0 || oo < 0 || to + len > length() || oo + len > other.length()) return false;
        for (int i = 0; i < len; i++) {
            char a = charAt(to + i), b = other.charAt(oo + i);
            if (a == b) continue;
            if (ic && (Character.toLowerCase(a) == Character.toLowerCase(b) || Character.toUpperCase(a) == Character.toUpperCase(b))) continue;
            return false;
        }
        return true;
    }
    public String repeat(int n) {
        StringBuilder sb = new StringBuilder(length() * n);
        for (int i = 0; i < n; i++) sb.append(this);
        return sb.toString();
    }
    public String toString() { return this; }

    public static String valueOf(Object o) { return o == null ? "null" : o.toString(); }
    public static String valueOf(char[] v) { return new String(v); }
    public static String valueOf(char[] v, int o, int c) { return new String(v, o, c); }
    public static String copyValueOf(char[] v) { return new String(v); }
    public static String copyValueOf(char[] v, int o, int c) { return new String(v, o, c); }
    public static String valueOf(boolean b) { return b ? "true" : "false"; }
    public static native String valueOf(char c);
    public static String valueOf(int i) { return Integer.toString(i); }
    public static String valueOf(long l) { return Long.toString(l); }
    public static String valueOf(float f) { return Float.toString(f); }
    public static String valueOf(double d) { return Double.toString(d); }
    public static native String format(String fmt, Object... args);
    public static String format(Locale l, String fmt, Object... args) { return format(fmt, args); }
    public static String join(CharSequence sep, CharSequence... parts) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < parts.length; i++) { if (i > 0) sb.append(sep); sb.append(parts[i]); }
        return sb.toString();
    }
    public static String join(CharSequence sep, Iterable<? extends CharSequence> parts) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (CharSequence p : parts) { if (!first) sb.append(sep); sb.append(p); first = false; }
        return sb.toString();
    }
}
