package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class Switch extends CompoundButton {
    public Switch(Context c) { this(c, null); }
    public Switch(Context c, AttributeSet a) { this(c, a, 16843839); }
    public Switch(Context c, AttributeSet a, int s) { super(c, a, s); }
    int idoridExtraWidth() { return text.length() == 0 ? 51 : 59; }
    int idoridExtraHeight() { return 31; }
    protected String idoridType() { return "Switch"; }
    public Switch(Context c, AttributeSet a, int s, int r) { this(c,a,s); }
}
