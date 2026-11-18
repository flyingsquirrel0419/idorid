package java.lang;

public final class StringBuffer extends AbstractStringBuilder implements java.io.Serializable {
    public StringBuffer() { super(16); }
    public StringBuffer(int cap) { super(cap); }
    public StringBuffer(String s) { super(s.length() + 16); appendStr(s); }
    public StringBuffer(CharSequence s) { super(s.length() + 16); appendSeq(s, 0, s.length()); }

    public StringBuffer append(Object o) { appendStr(String.valueOf(o)); return this; }
    public StringBuffer append(String s) { appendStr(s); return this; }
    public StringBuffer append(StringBuffer s) { appendStr(s == null ? null : s.toString()); return this; }
    public StringBuffer append(CharSequence s) { if (s == null) appendStr(null); else appendSeq(s, 0, s.length()); return this; }
    public StringBuffer append(CharSequence s, int b, int e) { appendSeq(s, b, e); return this; }
    public StringBuffer append(char[] s) { ensureCapacity(count + s.length); System.arraycopy(s, 0, value, count, s.length); count += s.length; return this; }
    public StringBuffer append(char[] s, int off, int len) { ensureCapacity(count + len); System.arraycopy(s, off, value, count, len); count += len; return this; }
    public StringBuffer append(boolean b) { appendStr(b ? "true" : "false"); return this; }
    public StringBuffer append(char c) { appendChar(c); return this; }
    public StringBuffer append(int i) { appendStr(Integer.toString(i)); return this; }
    public StringBuffer append(long l) { appendStr(Long.toString(l)); return this; }
    public StringBuffer append(float f) { appendStr(Float.toString(f)); return this; }
    public StringBuffer append(double d) { appendStr(Double.toString(d)); return this; }
    public StringBuffer appendCodePoint(int cp) {
        if (cp < 0x10000) appendChar((char) cp);
        else { appendChar(Character.highSurrogate(cp)); appendChar(Character.lowSurrogate(cp)); }
        return this;
    }
    public StringBuffer insert(int off, String s) { insertStr(off, s); return this; }
    public StringBuffer insert(int off, Object o) { insertStr(off, String.valueOf(o)); return this; }
    public StringBuffer insert(int off, char c) { insertStr(off, String.valueOf(c)); return this; }
    public StringBuffer insert(int off, int i) { insertStr(off, Integer.toString(i)); return this; }
    public StringBuffer insert(int off, long l) { insertStr(off, Long.toString(l)); return this; }
    public StringBuffer insert(int off, boolean b) { insertStr(off, String.valueOf(b)); return this; }
    public StringBuffer insert(int off, double d) { insertStr(off, Double.toString(d)); return this; }
    public StringBuffer insert(int off, char[] s) { insertStr(off, new String(s)); return this; }
    public StringBuffer insert(int off, CharSequence s) { insertStr(off, String.valueOf(s)); return this; }
    public StringBuffer delete(int b, int e) { deleteRange(b, e); return this; }
    public StringBuffer deleteCharAt(int i) {
        if (i < 0 || i >= count) throw new StringIndexOutOfBoundsException(i);
        deleteRange(i, i + 1); return this;
    }
    public StringBuffer replace(int b, int e, String s) { replaceRange(b, e, s); return this; }
    public StringBuffer reverse() { reverseInPlace(); return this; }
}
