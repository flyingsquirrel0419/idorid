package android.widget;
import android.content.Context;
import android.util.AttributeSet;
import android.view.View;
public class RadioGroup extends LinearLayout {
    public interface OnCheckedChangeListener { void onCheckedChanged(RadioGroup g, int checkedId); }
    private OnCheckedChangeListener listener;
    private int checkedId = -1;
    public RadioGroup(Context c) { super(c); setOrientation(VERTICAL); }
    public RadioGroup(Context c, AttributeSet a) { super(c, a); }
    void childChecked(CompoundButton b) {
        for (int i = 0; i < getChildCount(); i++) { View v = getChildAt(i); if (v != b && v instanceof CompoundButton) ((CompoundButton) v).setChecked(false); }
        checkedId = b.getId();
        if (listener != null) listener.onCheckedChanged(this, checkedId);
    }
    public void check(int id) { View v = findViewById(id); if (v instanceof CompoundButton) ((CompoundButton) v).setChecked(true); }
    public void clearCheck() { for (int i = 0; i < getChildCount(); i++) { View v = getChildAt(i); if (v instanceof CompoundButton) ((CompoundButton) v).setChecked(false); } checkedId = -1; }
    public int getCheckedRadioButtonId() { return checkedId; }
    public void setOnCheckedChangeListener(OnCheckedChangeListener l) { listener = l; }
}
