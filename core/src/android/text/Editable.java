package android.text;
public interface Editable extends CharSequence, Appendable {
    public static class Factory { private static final Factory INSTANCE=new Factory(); public static Factory getInstance(){return INSTANCE;} public Editable newEditable(CharSequence source){return new SpannableStringBuilder(source);} }
    Editable replace(int st, int en, CharSequence text);
    Editable insert(int where, CharSequence text);
    Editable delete(int st, int en);
    Editable append(CharSequence text);
    Editable append(CharSequence text, int start, int end);
    Editable append(char c);
    void clear();
    void clearSpans();
}
