package java.lang;

public final class StringBuilder extends AbstractStringBuilder implements java.io.Serializable {
    public StringBuilder() { super(16); }
    public StringBuilder(int cap) { super(cap); }
    public StringBuilder(String s) { super(s.length() + 16); appendStr(s); }
    public StringBuilder(CharSequence s) { super(s.length() + 16); appendSeq(s, 0, s.length()); }

    public StringBuilder append(Object o) { appendStr(String.valueOf(o)); return this; }
    public StringBuilder append(String s) { appendStr(s); return this; }
    public StringBuilder append(StringBuffer s) { appendStr(s == null ? null : s.toString()); return this; }
    public StringBuilder append(CharSequence s) { if (s == null) appendStr(null); else appendSeq(s, 0, s.length()); return this; }
    public StringBuilder append(CharSequence s, int b, int e) { appendSeq(s, b, e); return this; }
    public StringBuilder append(char[] s) { ensureCapacity(count + s.length); System.arraycopy(s, 0, value, count, s.length); count += s.length; return this; }
    public StringBuilder append(char[] s, int off, int len) { ensureCapacity(count + len); System.arraycopy(s, off, value, count, len); count += len; return this; }
    public StringBuilder append(boolean b) { appendStr(b ? "true" : "false"); return this; }
    public StringBuilder append(char c) { appendChar(c); return this; }
    public StringBuilder append(int i) { appendStr(Integer.toString(i)); return this; }
    public StringBuilder append(long l) { appendStr(Long.toString(l)); return this; }
    public StringBuilder append(float f) { appendStr(Float.toString(f)); return this; }
    public StringBuilder append(double d) { appendStr(Double.toString(d)); return this; }
    public StringBuilder appendCodePoint(int cp) {
        if (cp < 0x10000) appendChar((char) cp);
        else { appendChar(Character.highSurrogate(cp)); appendChar(Character.lowSurrogate(cp)); }
        return this;
    }
    public StringBuilder insert(int off, String s) { insertStr(off, s); return this; }
    public StringBuilder insert(int off, Object o) { insertStr(off, String.valueOf(o)); return this; }
    public StringBuilder insert(int off, char c) { insertStr(off, String.valueOf(c)); return this; }
    public StringBuilder insert(int off, int i) { insertStr(off, Integer.toString(i)); return this; }
    public StringBuilder insert(int off, long l) { insertStr(off, Long.toString(l)); return this; }
    public StringBuilder insert(int off, boolean b) { insertStr(off, String.valueOf(b)); return this; }
    public StringBuilder insert(int off, double d) { insertStr(off, Double.toString(d)); return this; }
    public StringBuilder insert(int off, char[] s) { insertStr(off, new String(s)); return this; }
    public StringBuilder insert(int off, CharSequence s) { insertStr(off, String.valueOf(s)); return this; }
    public StringBuilder delete(int b, int e) { deleteRange(b, e); return this; }
    public StringBuilder deleteCharAt(int i) {
        if (i < 0 || i >= count) throw new StringIndexOutOfBoundsException(i);
        deleteRange(i, i + 1); return this;
    }
    public StringBuilder replace(int b, int e, String s) { replaceRange(b, e, s); return this; }
    public StringBuilder reverse() { reverseInPlace(); return this; }
}
