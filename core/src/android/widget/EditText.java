package android.widget;
import android.content.Context;
import android.text.Editable;
import android.text.SpannableStringBuilder;
import android.util.AttributeSet;
public class EditText extends TextView {
    private SpannableStringBuilder editable;
    public EditText(Context c) { super(c); }
    public EditText(Context c, AttributeSet a) { super(c, a); }
    public EditText(Context c, AttributeSet a, int s) { super(c, a, s); }
    public EditText(Context c, AttributeSet a, int s, int r) { super(c, a, s, r); }
    void onSetText() { editable = null; }
    public Editable getText() {
        if (editable == null) {
            editable = new SpannableStringBuilder(text) {
                public SpannableStringBuilder replace(int st, int en, CharSequence t) { super.replace(st, en, t); EditText.this.text = toString(); return this; }
                public SpannableStringBuilder insert(int w, CharSequence t) { super.insert(w, t); EditText.this.text = toString(); return this; }
                public SpannableStringBuilder delete(int st, int en) { super.delete(st, en); EditText.this.text = toString(); return this; }
                public SpannableStringBuilder append(CharSequence t) { super.append(t); EditText.this.text = toString(); return this; }
                public SpannableStringBuilder append(char c) { super.append(c); EditText.this.text = toString(); return this; }
                public void clear() { super.clear(); EditText.this.text = ""; }
            };
        }
        return editable;
    }
    public void selectAll() {}
    public void setSelection(int start, int stop) {}
    public void extendSelection(int i) {}
    protected String idoridType() { return "EditText"; }
}
