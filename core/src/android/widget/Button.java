package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class Button extends TextView {
    public Button(Context c) { super(c); }
    public Button(Context c, AttributeSet a) { super(c, a); }
    public Button(Context c, AttributeSet a, int s) { super(c, a, s); }
    public Button(Context c, AttributeSet a, int s, int r) { super(c, a, s, r); }
    protected String idoridType() { return "Button"; }
}
