package java.lang;

abstract class AbstractStringBuilder implements Appendable, CharSequence {
    char[] value;
    int count;

    AbstractStringBuilder(int cap) { value = new char[cap]; }

    final char[] getValue() { return value; }
    public int length() { return count; }
    public int capacity() { return value.length; }
    public char charAt(int i) {
        if (i < 0 || i >= count) throw new StringIndexOutOfBoundsException(i);
        return value[i];
    }
    public void setCharAt(int i, char c) {
        if (i < 0 || i >= count) throw new StringIndexOutOfBoundsException(i);
        value[i] = c;
    }
    public void ensureCapacity(int min) {
        if (min > value.length) {
            int n = value.length * 2 + 2;
            if (n < min) n = min;
            char[] nv = new char[n];
            System.arraycopy(value, 0, nv, 0, count);
            value = nv;
        }
    }
    public void setLength(int n) {
        ensureCapacity(n);
        for (int i = count; i < n; i++) value[i] = 0;
        count = n;
    }
    public void trimToSize() {}
    final void appendStr(String s) {
        if (s == null) s = "null";
        int n = s.length();
        ensureCapacity(count + n);
        s.getChars(0, n, value, count);
        count += n;
    }
    final void appendChar(char c) {
        if (count == value.length) ensureCapacity(count + 1);
        value[count++] = c;
    }
    final void appendSeq(CharSequence s, int b, int e) {
        if (s == null) { appendStr("null"); return; }
        ensureCapacity(count + e - b);
        for (int i = b; i < e; i++) value[count++] = s.charAt(i);
    }
    final void insertStr(int off, String s) {
        if (off < 0 || off > count) throw new StringIndexOutOfBoundsException(off);
        if (s == null) s = "null";
        int n = s.length();
        ensureCapacity(count + n);
        System.arraycopy(value, off, value, off + n, count - off);
        s.getChars(0, n, value, off);
        count += n;
    }
    final void deleteRange(int b, int e) {
        if (e > count) e = count;
        if (b < 0 || b > e) throw new StringIndexOutOfBoundsException(b);
        System.arraycopy(value, e, value, b, count - e);
        count -= e - b;
    }
    final void reverseInPlace() {
        for (int i = 0, j = count - 1; i < j; i++, j--) { char t = value[i]; value[i] = value[j]; value[j] = t; }
    }
    final void replaceRange(int b, int e, String s) {
        if (e > count) e = count;
        deleteRange(b, e);
        insertStr(b, s);
    }
    public int indexOf(String s) { return toString().indexOf(s); }
    public int indexOf(String s, int from) { return toString().indexOf(s, from); }
    public int lastIndexOf(String s) { return toString().lastIndexOf(s); }
    public String substring(int b) { return substring(b, count); }
    public String substring(int b, int e) {
        if (b < 0 || e > count || b > e) throw new StringIndexOutOfBoundsException(b);
        return new String(value, b, e - b);
    }
    public CharSequence subSequence(int b, int e) { return substring(b, e); }
    public void getChars(int sb, int se, char[] dst, int db) { System.arraycopy(value, sb, dst, db, se - sb); }
    public String toString() { return new String(value, 0, count); }
}
