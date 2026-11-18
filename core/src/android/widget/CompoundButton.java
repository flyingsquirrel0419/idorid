package android.widget;
import android.content.Context;
import android.util.AttributeSet;
import android.view.Json;
import android.view.XmlAttrs;
public abstract class CompoundButton extends Button implements Checkable {
    public interface OnCheckedChangeListener { void onCheckedChanged(CompoundButton b, boolean checked); }
    private boolean checked;
    private OnCheckedChangeListener listener;
    public CompoundButton(Context c) { super(c); }
    public CompoundButton(Context c, AttributeSet a) { this(c, a, 0); }
    public CompoundButton(Context c, AttributeSet a, int s) {
        super(c, a, s);
        if (a instanceof XmlAttrs) checked = ((XmlAttrs) a).bool("checked", false);
    }
    public boolean isChecked() { return checked; }
    public void setChecked(boolean c) {
        if (checked == c) return;
        checked = c;
        invalidate();
        if (listener != null) listener.onCheckedChanged(this, c);
        if (getParent() instanceof RadioGroup && c) ((RadioGroup) getParent()).childChecked(this);
    }
    public void toggle() { setChecked(!checked); }
    public boolean performClick() { toggle(); return super.performClick(); }
    public void setOnCheckedChangeListener(OnCheckedChangeListener l) { listener = l; }
    public void setButtonDrawable(int r) {}
    void buttonDefaults() {
        if (getMinimumHeight() == 0) setMinimumHeight(48);
        if (getGravity() == 0) setGravity(android.view.Gravity.START | android.view.Gravity.CENTER_VERTICAL);
    }
    int idoridExtraWidth() { return text.length() == 0 ? 32 : 40; }
    int idoridExtraHeight() { return 32; }
    protected void idoridProps(StringBuilder sb) { super.idoridProps(sb); Json.key(sb, "checked"); sb.append(checked); Json.key(sb, "clickable"); sb.append("true"); }
}
