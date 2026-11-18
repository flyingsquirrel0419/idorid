package java.lang;
public final class Character implements java.io.Serializable, Comparable<Character> {
    public static final char MIN_VALUE = '\u0000';
    public static final char MAX_VALUE = '￿';
    public static final int MIN_RADIX = 2;
    public static final int MAX_RADIX = 36;
    public static final char MIN_HIGH_SURROGATE = '\uD800';
    public static final char MAX_HIGH_SURROGATE = '\uDBFF';
    public static final char MIN_LOW_SURROGATE = '\uDC00';
    public static final char MAX_LOW_SURROGATE = '\uDFFF';
    public static final int MIN_SUPPLEMENTARY_CODE_POINT = 0x10000;
    public static final int MAX_CODE_POINT = 0x10ffff;
    public static final int SIZE = 16;
    public static final int BYTES = 2;
    @SuppressWarnings("unchecked")
    public static final Class<Character> TYPE = (Class<Character>) VMPrimitive.get('C');
    private static final Character[] CACHE = new Character[128];
    private final char value;
    public Character(char c) { value = c; }
    public static Character valueOf(char c) {
        if (c < 128) {
            Character x = CACHE[c];
            if (x == null) { x = new Character(c); CACHE[c] = x; }
            return x;
        }
        return new Character(c);
    }
    public char charValue() { return value; }
    public static String toString(char c) { return String.valueOf(c); }
    public static boolean isDigit(char c) { return isDigit((int) c); }
    public static boolean isDigit(int c) { return (c >= '0' && c <= '9') || (c >= 0x660 && c <= 0x669) || (c >= 0xff10 && c <= 0xff19); }
    public static boolean isLetter(char c) { return isLetter((int) c); }
    public static native boolean isLetter(int c);
    public static boolean isLetterOrDigit(char c) { return isLetter(c) || isDigit(c); }
    public static boolean isLetterOrDigit(int c) { return isLetter(c) || isDigit(c); }
    public static boolean isAlphabetic(int c) { return isLetter(c); }
    public static boolean isUpperCase(char c) { return isUpperCase((int) c); }
    public static native boolean isUpperCase(int c);
    public static boolean isLowerCase(char c) { return isLowerCase((int) c); }
    public static native boolean isLowerCase(int c);
    public static boolean isWhitespace(char c) { return isWhitespace((int) c); }
    public static boolean isWhitespace(int c) {
        return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == 0x0b || c == 0x0c || (c >= 0x1c && c <= 0x1f)
            || c == 0x1680 || (c >= 0x2000 && c <= 0x2006) || (c >= 0x2008 && c <= 0x200a) || c == 0x2028 || c == 0x2029 || c == 0x205f || c == 0x3000;
    }
    public static boolean isSpaceChar(char c) { return c == ' ' || c == 0xa0 || isWhitespace(c); }
    public static boolean isISOControl(char c) { return c <= 0x1f || (c >= 0x7f && c <= 0x9f); }
    public static char toUpperCase(char c) { return (char) toUpperCase((int) c); }
    public static native int toUpperCase(int c);
    public static char toLowerCase(char c) { return (char) toLowerCase((int) c); }
    public static native int toLowerCase(int c);
    public static int digit(char c, int radix) { return digit((int) c, radix); }
    public static int digit(int c, int radix) {
        int d;
        if (c >= '0' && c <= '9') d = c - '0';
        else if (c >= 'a' && c <= 'z') d = c - 'a' + 10;
        else if (c >= 'A' && c <= 'Z') d = c - 'A' + 10;
        else return -1;
        return d < radix ? d : -1;
    }
    public static int getNumericValue(char c) { return digit(c, 36); }
    public static char forDigit(int d, int radix) {
        if (d < 0 || d >= radix || radix < 2 || radix > 36) return 0;
        return (char) (d < 10 ? '0' + d : 'a' + d - 10);
    }
    public static boolean isHighSurrogate(char c) { return c >= MIN_HIGH_SURROGATE && c <= MAX_HIGH_SURROGATE; }
    public static boolean isLowSurrogate(char c) { return c >= MIN_LOW_SURROGATE && c <= MAX_LOW_SURROGATE; }
    public static boolean isSurrogate(char c) { return c >= MIN_HIGH_SURROGATE && c <= MAX_LOW_SURROGATE; }
    public static boolean isSurrogatePair(char h, char l) { return isHighSurrogate(h) && isLowSurrogate(l); }
    public static int toCodePoint(char h, char l) { return ((h - 0xd800) << 10) + (l - 0xdc00) + 0x10000; }
    public static char highSurrogate(int cp) { return (char) ((cp >>> 10) + (0xd800 - (0x10000 >>> 10))); }
    public static char lowSurrogate(int cp) { return (char) ((cp & 0x3ff) + 0xdc00); }
    public static int charCount(int cp) { return cp >= 0x10000 ? 2 : 1; }
    public static boolean isBmpCodePoint(int cp) { return cp >>> 16 == 0; }
    public static boolean isValidCodePoint(int cp) { return cp >= 0 && cp <= MAX_CODE_POINT; }
    public static boolean isSupplementaryCodePoint(int cp) { return cp >= 0x10000 && cp <= MAX_CODE_POINT; }
    public static int toChars(int cp,char[] dst,int index) { char[] chars=toChars(cp);for(int i=0;i<chars.length;i++)dst[index+i]=chars[i];return chars.length; }
    public static int codePointBefore(CharSequence s,int index) { char low=s.charAt(index-1);if(isLowSurrogate(low)&&index>1){char high=s.charAt(index-2);if(isHighSurrogate(high))return toCodePoint(high,low);}return low; }
    public static byte getDirectionality(char c) { return getDirectionality((int)c); }
    public static byte getDirectionality(int c) { if(c>=0x590&&c<=0x5ff)return 1; if(c>=0x600&&c<=0x8ff)return 2; if(c>='0'&&c<='9')return 3; if(c==' '||c=='\t'||c=='\n')return 12; return 0; }
    public static char[] toChars(int cp) { return cp < 0x10000 ? new char[] { (char) cp } : new char[] { highSurrogate(cp), lowSurrogate(cp) }; }
    public static String toString(int cp) { return new String(toChars(cp)); }
    public static int codePointAt(CharSequence s, int i) {
        char h = s.charAt(i);
        if (isHighSurrogate(h) && i + 1 < s.length()) { char l = s.charAt(i + 1); if (isLowSurrogate(l)) return toCodePoint(h, l); }
        return h;
    }
    public static int codePointAt(char[] a, int i) {
        char h = a[i];
        if (isHighSurrogate(h) && i + 1 < a.length && isLowSurrogate(a[i + 1])) return toCodePoint(h, a[i + 1]);
        return h;
    }
    public static int compare(char a, char b) { return a - b; }
    public static int hashCode(char c) { return c; }
    public static char reverseBytes(char c) { return (char) (((c & 0xff00) >> 8) | (c << 8)); }
    public int hashCode() { return value; }
    public boolean equals(Object o) { return o instanceof Character && ((Character) o).value == value; }
    public int compareTo(Character o) { return value - o.value; }
    public String toString() { return String.valueOf(value); }
}
