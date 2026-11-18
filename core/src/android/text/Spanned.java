package android.text;
public interface Spanned extends CharSequence {
    int SPAN_EXCLUSIVE_EXCLUSIVE = 33, SPAN_INCLUSIVE_EXCLUSIVE = 17, SPAN_INCLUSIVE_INCLUSIVE = 18, SPAN_EXCLUSIVE_INCLUSIVE = 34;
    <T> T[] getSpans(int start, int end, Class<T> type);
    int getSpanStart(Object tag); int getSpanEnd(Object tag); int getSpanFlags(Object tag);
    int nextSpanTransition(int start, int limit, Class type);
}
