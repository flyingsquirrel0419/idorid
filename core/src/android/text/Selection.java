package android.text;
public final class Selection {
    public static final Object SELECTION_START=new Object(),SELECTION_END=new Object();
    public static int getSelectionStart(CharSequence text){return text instanceof Spanned?((Spanned)text).getSpanStart(SELECTION_START):-1;} public static int getSelectionEnd(CharSequence text){return text instanceof Spanned?((Spanned)text).getSpanStart(SELECTION_END):-1;}
    public static void setSelection(Spannable text,int start,int end){text.setSpan(SELECTION_START,start,start,Spanned.SPAN_INCLUSIVE_INCLUSIVE);text.setSpan(SELECTION_END,end,end,Spanned.SPAN_INCLUSIVE_INCLUSIVE);} public static void setSelection(Spannable text,int index){setSelection(text,index,index);} public static void selectAll(Spannable text){setSelection(text,0,text.length());} public static void removeSelection(Spannable text){text.removeSpan(SELECTION_START);text.removeSpan(SELECTION_END);}
}
