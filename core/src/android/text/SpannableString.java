package android.text;
public class SpannableString implements Spannable {
    private final SpannableStringBuilder data;
    public SpannableString(CharSequence s) { data=new SpannableStringBuilder(s); }
    public static SpannableString valueOf(CharSequence s) { return new SpannableString(s); }
    public int length() { return data.length(); } public char charAt(int i) { return data.charAt(i); }
    public CharSequence subSequence(int a,int b) { return new SpannableString(data.subSequence(a,b)); }
    public String toString() { return data.toString(); }
    public void setSpan(Object what,int a,int b,int f) { data.setSpan(what,a,b,f); }
    public void removeSpan(Object what) { data.removeSpan(what); }
    public <T> T[] getSpans(int a,int b,Class<T> c) { return data.getSpans(a,b,c); }
    public int getSpanStart(Object o) { return data.getSpanStart(o); } public int getSpanEnd(Object o) { return data.getSpanEnd(o); } public int getSpanFlags(Object o) { return data.getSpanFlags(o); }
    public int nextSpanTransition(int start,int limit,Class type) { return data.nextSpanTransition(start,limit,type); }
}
