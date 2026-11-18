package android.text;
public interface SpanWatcher { void onSpanAdded(Spannable text,Object what,int start,int end); void onSpanRemoved(Spannable text,Object what,int start,int end); void onSpanChanged(Spannable text,Object what,int oldStart,int oldEnd,int newStart,int newEnd); }
