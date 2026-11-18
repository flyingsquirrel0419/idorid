package android.text;
import java.util.ArrayList;
public class SpannableStringBuilder implements Editable, Spannable {
    private final StringBuilder sb;
    private final ArrayList<Span> spans = new ArrayList<Span>();
    private static class Span { Object obj; int start,end,flags; Span(Object o,int s,int e,int f) { obj=o; start=s; end=e; flags=f; } }
    public SpannableStringBuilder() { sb = new StringBuilder(); }
    public SpannableStringBuilder(CharSequence s) { this(s,0,s == null ? 0 : s.length()); }
    public SpannableStringBuilder(CharSequence s,int start,int end) { sb=new StringBuilder(s == null ? "" : s.subSequence(start,end).toString()); if (s instanceof Spanned) { Spanned p=(Spanned)s; for (Object o : p.getSpans(start,end,Object.class)) setSpan(o,Math.max(0,p.getSpanStart(o)-start),Math.min(end-start,p.getSpanEnd(o)-start),p.getSpanFlags(o)); } }
    public int length() { return sb.length(); } public char charAt(int i) { return sb.charAt(i); }
    public CharSequence subSequence(int s, int e) { return new SpannableStringBuilder(this,s,e); }
    public String toString() { return sb.toString(); }
    private void adjust(int start,int end,int len) { int delta=len-(end-start); for (Span s : spans) { s.start=s.start>end ? s.start+delta : s.start>=start ? start : s.start; s.end=s.end>end ? s.end+delta : s.end>=start ? start+len : s.end; if (s.end<s.start) s.end=s.start; } }
    public SpannableStringBuilder replace(int st,int en,CharSequence t) { return replace(st,en,t,0,t.length()); }
    public SpannableStringBuilder replace(int st,int en,CharSequence t,int start,int end) { String text=t.subSequence(start,end).toString(); adjust(st,en,text.length()); sb.replace(st,en,text); return this; }
    public SpannableStringBuilder insert(int w,CharSequence t) { return insert(w,t,0,t.length()); }
    public SpannableStringBuilder insert(int w,CharSequence t,int start,int end) { return replace(w,w,t,start,end); }
    public SpannableStringBuilder delete(int st,int en) { return replace(st,en,""); }
    public SpannableStringBuilder append(CharSequence t) { return insert(length(),t); }
    public SpannableStringBuilder append(CharSequence t,int s,int e) { return insert(length(),t,s,e); }
    public SpannableStringBuilder append(char c) { return append(String.valueOf(c)); }
    public SpannableStringBuilder append(CharSequence t,Object what,int flags) { int s=length(); append(t); setSpan(what,s,length(),flags); return this; }
    public void clear() { delete(0,length()); } public void clearSpans() { spans.clear(); }
    public void setSpan(Object what,int s,int e,int f) { if (s<0 || e<s || e>length()) throw new IndexOutOfBoundsException(); removeSpan(what); spans.add(new Span(what,s,e,f)); }
    public void removeSpan(Object what) { for (int i=spans.size()-1;i>=0;i--) if (spans.get(i).obj==what) spans.remove(i); }
    private Span find(Object o) { for (Span s:spans) if (s.obj==o) return s; return null; }
    public int getSpanStart(Object o) { Span s=find(o); return s==null ? -1:s.start; }
    public int getSpanEnd(Object o) { Span s=find(o); return s==null ? -1:s.end; }
    public int getSpanFlags(Object o) { Span s=find(o); return s==null ? 0:s.flags; }
    public <T> T[] getSpans(int start,int end,Class<T> type) { ArrayList<T> result=new ArrayList<T>(); for (Span s:spans) if (type.isInstance(s.obj) && s.start<=end && s.end>=start && (s.start==s.end || start==end || (s.start<end && s.end>start))) result.add(type.cast(s.obj)); T[] a=(T[])java.lang.reflect.Array.newInstance(type,result.size()); return result.toArray(a); }
    public int nextSpanTransition(int start,int limit,Class type) { for (Span s:spans) if (type==null || type.isInstance(s.obj)) { if (s.start>start && s.start<limit) limit=s.start; if (s.end>start && s.end<limit) limit=s.end; } return limit; }
    public boolean equals(Object o) { return o instanceof CharSequence && toString().equals(o.toString()); } public int hashCode() { return toString().hashCode(); }
}
