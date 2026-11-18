package android.widget;
public abstract class AbsSpinner extends AdapterView<SpinnerAdapter> { public AbsSpinner(android.content.Context c) { super(c); } public AbsSpinner(android.content.Context c,android.util.AttributeSet a) { super(c,a); } public AbsSpinner(android.content.Context c,android.util.AttributeSet a,int s) { super(c,a,s); } public AbsSpinner(android.content.Context c,android.util.AttributeSet a,int s,int r) { super(c,a,s,r); }  public abstract SpinnerAdapter getAdapter();
}
