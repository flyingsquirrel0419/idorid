package android.widget;

import android.content.Context;
import android.graphics.Typeface;
import android.text.Editable;
import android.text.SpannableStringBuilder;
import android.text.TextWatcher;
import android.util.AttributeSet;
import android.view.Gravity;
import android.view.Json;
import android.view.View;
import android.view.XmlAttrs;
import java.util.ArrayList;

public class TextView extends View {
    public enum BufferType { NORMAL, SPANNABLE, EDITABLE }
    public interface OnEditorActionListener { boolean onEditorAction(TextView v, int actionId, android.view.KeyEvent event); }
    CharSequence text = "";
    private CharSequence hint;
    private float textSize = 14f;
    private int textColor;
    private boolean hasTextColor;
    private int hintColor;
    private int gravity;
    private int style;
    private boolean allCaps;
    private int maxLines = -1;
    private int inputType;
    private CharSequence error;
    ArrayList<TextWatcher> watchers;
    private OnEditorActionListener editorListener;

    public TextView(Context c) { super(c); }
    public TextView(Context c, AttributeSet a) { this(c, a, 0); }
    public TextView(Context c, AttributeSet a, int s) { this(c, a, s, 0); }
    public TextView(Context c, AttributeSet a, int s, int r) {
        super(c, a, s, r);
        if (a instanceof XmlAttrs) {
            XmlAttrs x = (XmlAttrs) a;
            String t = x.string("text");
            if (t != null) text = t;
            hint = x.string("hint");
            textSize = x.dimension("textSize", textSize);
            if (x.has("textColor")) { textColor = x.integer("textColor", 0); hasTextColor = true; }
            if (x.has("textColorHint")) hintColor = x.integer("textColorHint", 0);
            gravity = x.integer("gravity", 0);
            String ts = x.string("textStyle");
            if (ts != null) style = ts.contains("bold") || ts.equals("1") || ts.equals("3") ? Typeface.BOLD | (ts.contains("italic") || ts.equals("3") ? 2 : 0) : ts.contains("italic") || ts.equals("2") ? Typeface.ITALIC : 0;
            allCaps = x.bool("textAllCaps", false);
            maxLines = x.integer("maxLines", x.bool("singleLine", false) ? 1 : x.integer("lines", -1));
            inputType = x.integer("inputType", 0);
        }
    }
    public void setText(CharSequence t) { setText(t, BufferType.NORMAL); }
    public void setText(CharSequence t, BufferType type) {
        CharSequence old = text;
        requestLayout();
        if (watchers != null) for (TextWatcher w : new ArrayList<TextWatcher>(watchers)) w.beforeTextChanged(old, 0, old.length(), t == null ? 0 : t.length());
        text = t == null ? "" : t;
        onSetText();
        if (watchers != null) {
            for (TextWatcher w : new ArrayList<TextWatcher>(watchers)) w.onTextChanged(text, 0, old.length(), text.length());
            Editable e = this instanceof EditText ? ((EditText) this).getText() : new SpannableStringBuilder(text);
            for (TextWatcher w : new ArrayList<TextWatcher>(watchers)) w.afterTextChanged(e);
        }
    }
    void onSetText() {}
    public final void setText(int resid) { setText(getContext().getResources().getText(resid)); }
    public final void setText(char[] t, int start, int len) { setText(new String(t, start, len)); }
    public CharSequence getText() { return text; }
    public void append(CharSequence t) { setText(text.toString() + t); }
    public int length() { return text.length(); }
    public void setHint(CharSequence h) { hint = h; requestLayout(); }
    public final void setHint(int resid) { hint = getContext().getResources().getText(resid); }
    public CharSequence getHint() { return hint; }
    public void setTextSize(float sp) { textSize = sp; requestLayout(); }
    public void setTextSize(int unit, float size) { textSize = unit == 0 ? size : size; requestLayout(); }
    public float getTextSize() { return textSize; }
    public void setTextColor(int c) { textColor = c; hasTextColor = true; }
    public void setTextColor(android.content.res.ColorStateList c) { setTextColor(c.getDefaultColor()); }
    public final int getCurrentTextColor() { return hasTextColor ? textColor : 0xde000000; }
    public void setHintTextColor(int c) { hintColor = c; }
    public void setGravity(int g) { gravity = g; }
    public int getGravity() { return gravity; }
    public void setTypeface(Typeface tf) { style = tf == null ? 0 : tf.getStyle(); requestLayout(); }
    public void setTypeface(Typeface tf, int s) { style = s; requestLayout(); }
    public Typeface getTypeface() { return Typeface.defaultFromStyle(style); }
    public void setAllCaps(boolean b) { allCaps = b; requestLayout(); }
    public void setMaxLines(int n) { maxLines = n; requestLayout(); }
    public void setLines(int n) { maxLines = n; requestLayout(); }
    public void setMinLines(int n) {}
    public void setSingleLine() { maxLines = 1; requestLayout(); }
    public void setSingleLine(boolean b) { maxLines = b ? 1 : -1; requestLayout(); }
    public void setEllipsize(android.text.TextUtils.TruncateAt w) {}
    public void setLineSpacing(float add, float mult) {}
    public void setLetterSpacing(float s) {}
    public void setIncludeFontPadding(boolean b) {}
    public void setMovementMethod(android.text.method.MovementMethod m) {}
    public void setTransformationMethod(android.text.method.TransformationMethod m) {
        if (m instanceof android.text.method.PasswordTransformationMethod) inputType = 0x81; else if (inputType == 0x81) inputType = 1;
    }
    public void setInputType(int t) { inputType = t; }
    public int getInputType() { return inputType; }
    public void setImeOptions(int o) {}
    public void setOnEditorActionListener(OnEditorActionListener l) { editorListener = l; }
    public void setError(CharSequence e) { error = e; }
    public CharSequence getError() { return error; }
    public void setCompoundDrawablesWithIntrinsicBounds(int l, int t, int r, int b) {}
    public void setCompoundDrawables(android.graphics.drawable.Drawable l, android.graphics.drawable.Drawable t, android.graphics.drawable.Drawable r, android.graphics.drawable.Drawable b) {}
    public void setCompoundDrawablePadding(int p) {}
    public void setHorizontallyScrolling(boolean b) {}
    public void setTextIsSelectable(boolean b) {}
    public void setCursorVisible(boolean b) {}
    public void setSelection(int i) {}
    public void addTextChangedListener(TextWatcher w) { if (watchers == null) watchers = new ArrayList<TextWatcher>(); watchers.add(w); }
    public void removeTextChangedListener(TextWatcher w) { if (watchers != null) watchers.remove(w); }
    /** idorid: user typed in the host field. */
    public void idoridUserText(String t) { setText(t); }
    /** idorid: return key pressed. */
    public void idoridEditorAction() { if (editorListener != null) editorListener.onEditorAction(this, 6, null); }

    /** Text as shown (all-caps and password transforms applied). */
    CharSequence displayText() {
        CharSequence t = text.length() > 0 ? text : (hint != null ? hint : "");
        if (allCaps) t = t.toString().toUpperCase();
        if ((inputType & 0xfff) == 0x81 || (inputType & 0xfff) == 0x12 || (inputType & 0xfff) == 0xe1) {
            StringBuilder b = new StringBuilder(); for (int i = 0; i < t.length(); i++) b.append('\u2022'); t = b;
        }
        return t;
    }
    int textFlags() {
        int f = (style & 1) != 0 ? android.view.VMText.BOLD : 0;
        if ((style & 2) != 0) f |= android.view.VMText.ITALIC;
        if (maxLines > 0) f |= Math.min(maxLines, 255) << 8;
        return f;
    }
    protected void onMeasure(int ws, int hs) {
        int wMode = MeasureSpec.getMode(ws), wSize = MeasureSpec.getSize(ws);
        int hMode = MeasureSpec.getMode(hs), hSize = MeasureSpec.getSize(hs);
        String shown = displayText().toString();
        int padH = getPaddingLeft() + getPaddingRight(), padV = getPaddingTop() + getPaddingBottom();
        int extraW = idoridExtraWidth();
        int width;
        if (wMode == MeasureSpec.EXACTLY) width = wSize;
        else {
            float max = wMode == MeasureSpec.AT_MOST ? Math.max(0, wSize - padH - extraW) : 0;
            long m = android.view.VMText.measure(shown, textSize, textFlags(), max);
            width = android.view.VMText.widthOf(m) + padH + extraW;
            width = Math.max(width, getSuggestedMinimumWidth());
            if (wMode == MeasureSpec.AT_MOST) width = Math.min(width, wSize);
        }
        long m = android.view.VMText.measure(shown, textSize, textFlags(), Math.max(1, width - padH - extraW));
        int height;
        if (hMode == MeasureSpec.EXACTLY) height = hSize;
        else {
            height = Math.max(android.view.VMText.heightOf(m), idoridExtraHeight()) + padV;
            height = Math.max(height, getSuggestedMinimumHeight());
            if (hMode == MeasureSpec.AT_MOST) height = Math.min(height, hSize);
        }
        setMeasuredDimension(width, height);
    }
    /** idorid: room for a compound button / switch beside the text. */
    int idoridExtraWidth() { return 0; }
    int idoridExtraHeight() { return 0; }
    public int getLineHeight() { return Math.round(textSize * 1.17f); }
    public int getLineCount() { return Math.max(1, android.view.VMText.heightOf(android.view.VMText.measure(displayText().toString(), textSize, textFlags(), Math.max(1, getWidth() - getPaddingLeft() - getPaddingRight()))) / Math.max(1, getLineHeight())); }
    public int getBaseline() { return getPaddingTop() + Math.round(textSize * 0.93f); }
    public int getMaxLines() { return maxLines; }
    public boolean isAllCaps() { return allCaps; }
    protected String idoridType() { return "TextView"; }
    protected void idoridProps(StringBuilder sb) {
        Json.key(sb, "text"); Json.str(sb, allCaps ? text.toString().toUpperCase() : text);
        if (hint != null) { Json.key(sb, "hint"); Json.str(sb, hint); }
        Json.key(sb, "textSize"); Json.num(sb, textSize);
        if (hasTextColor) { Json.key(sb, "color"); Json.str(sb, android.graphics.Color.toHex(textColor)); }
        if (hintColor != 0) { Json.key(sb, "hintColor"); Json.str(sb, android.graphics.Color.toHex(hintColor)); }
        String g = Gravity.describe(gravity);
        if (g != null) { Json.key(sb, "gravity"); Json.str(sb, g); }
        if ((style & 1) != 0) { Json.key(sb, "bold"); sb.append("true"); }
        if ((style & 2) != 0) { Json.key(sb, "italic"); sb.append("true"); }
        if (maxLines > 0) { Json.key(sb, "maxLines"); sb.append(maxLines); }
        if (inputType != 0) { Json.key(sb, "inputType"); sb.append(inputType); }
        if (error != null) { Json.key(sb, "errorText"); Json.str(sb, error); }
    }
}
