package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class RadioButton extends CompoundButton {
    public RadioButton(Context c) { super(c); }
    public RadioButton(Context c, AttributeSet a) { super(c, a); }
    public RadioButton(Context c, AttributeSet a, int s) { super(c, a, s); }
    public void toggle() { if (!isChecked()) super.toggle(); }
    protected String idoridType() { return "RadioButton"; }
}
