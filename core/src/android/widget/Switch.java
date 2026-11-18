package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class Switch extends CompoundButton {
    public Switch(Context c) { super(c); }
    public Switch(Context c, AttributeSet a) { super(c, a); }
    public Switch(Context c, AttributeSet a, int s) { super(c, a, s); }
    protected String idoridType() { return "Switch"; }
}
