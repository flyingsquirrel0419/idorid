package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class CheckBox extends CompoundButton {
    public CheckBox(Context c) { super(c); }
    public CheckBox(Context c, AttributeSet a) { super(c, a); }
    public CheckBox(Context c, AttributeSet a, int s) { super(c, a, s); }
    protected String idoridType() { return "CheckBox"; }
}
