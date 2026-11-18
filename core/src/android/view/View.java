package android.view;

import android.content.Context;
import android.content.res.Resources;
import android.graphics.drawable.ColorDrawable;
import android.graphics.drawable.Drawable;
import android.os.Handler;
import android.util.AttributeSet;

public class View {
    public static final int VISIBLE = 0, INVISIBLE = 4, GONE = 8, NO_ID = -1;
    public static final int TEXT_ALIGNMENT_CENTER = 4, TEXT_ALIGNMENT_VIEW_START = 5, TEXT_ALIGNMENT_GRAVITY = 1;
    public interface OnClickListener { void onClick(View v); }
    public interface OnLongClickListener { boolean onLongClick(View v); }
    public interface OnTouchListener { boolean onTouch(View v, MotionEvent e); }
    public interface OnFocusChangeListener { void onFocusChange(View v, boolean hasFocus); }
    public interface OnKeyListener { boolean onKey(View v, int keyCode, KeyEvent e); }
    public interface OnLayoutChangeListener { void onLayoutChange(View v, int l, int t, int r, int b, int ol, int ot, int or, int ob); }
    public interface OnAttachStateChangeListener { void onViewAttachedToWindow(View v); void onViewDetachedFromWindow(View v); }

    private static int nextUid = 1;
    private static final Handler HANDLER = new Handler();
    /** Host-visible identity (stable for the life of the view). */
    public final int idoridUid = nextUid++;
    private final Context mContext;
    private int mId = NO_ID;
    ViewParent mParent;
    private ViewGroup.LayoutParams mLayoutParams;
    private int visibility = VISIBLE;
    private boolean enabled = true, clickable, selected, focusable;
    int pl, pt, pr, pb;
    private int bgColor;
    private int bgRes;
    private float alpha = 1f, cornerRadius;
    private int minW, minH;
    private Object tag;
    private android.util.SparseArray<Object> keyedTags;
    private CharSequence contentDescription;
    protected OnClickListener mOnClickListener;
    private OnLongClickListener longClick;
    private OnFocusChangeListener focusListener;

    public View(Context context) { mContext = context; }
    public View(Context context, AttributeSet attrs) { this(context, attrs, 0); }
    public View(Context context, AttributeSet attrs, int defStyle) { this(context, attrs, defStyle, 0); }
    public View(Context context, AttributeSet attrs, int defStyleAttr, int defStyleRes) {
        mContext = context;
        if (attrs instanceof XmlAttrs) applyAttrs((XmlAttrs) attrs);
    }

    private void applyAttrs(XmlAttrs a) {
        mId = a.integer("id", NO_ID);
        int p = (int) a.dimension("padding", -1);
        if (p >= 0) pl = pt = pr = pb = p;
        int ph = (int) a.dimension("paddingHorizontal", -1), pv = (int) a.dimension("paddingVertical", -1);
        if (ph >= 0) pl = pr = ph;
        if (pv >= 0) pt = pb = pv;
        pl = (int) a.dimension("paddingLeft", a.dimension("paddingStart", pl));
        pr = (int) a.dimension("paddingRight", a.dimension("paddingEnd", pr));
        pt = (int) a.dimension("paddingTop", pt);
        pb = (int) a.dimension("paddingBottom", pb);
        String bg = a.raw("background");
        if (bg != null) { if (bg.startsWith("c:")) bgColor = a.integer("background", 0); else if (bg.startsWith("r:")) bgRes = a.integer("background", 0); }
        String vis = a.string("visibility");
        if (vis != null) visibility = vis.equals("gone") || vis.equals("2") ? GONE : vis.equals("invisible") || vis.equals("1") ? INVISIBLE : VISIBLE;
        enabled = a.bool("enabled", true);
        clickable = a.bool("clickable", false);
        alpha = a.dimension("alpha", 1f);
        minW = (int) a.dimension("minWidth", 0);
        minH = (int) a.dimension("minHeight", 0);
        contentDescription = a.string("contentDescription");
        String onClick = a.string("onClick");
        if (onClick != null) {
            final String method = onClick;
            mOnClickListener = new OnClickListener() {
                public void onClick(View v) { VMBridge.invokeOnClick(getContext(), method, v); }
            };
        }
    }

    public Context getContext() { return mContext; }
    public Resources getResources() { return mContext.getResources(); }
    public int getId() { return mId; }
    public void setId(int id) { mId = id; }
    public static int generateViewId() { return 0x00ff0000 + nextUid++; }
    @SuppressWarnings("unchecked")
    public final <T extends View> T findViewById(int id) { return id == NO_ID ? null : (T) findViewTraversal(id); }
    @SuppressWarnings("unchecked")
    public final <T extends View> T requireViewById(int id) {
        T v = findViewById(id);
        if (v == null) throw new IllegalArgumentException("ID does not reference a View inside this View");
        return v;
    }
    @SuppressWarnings("unchecked")
    public final <T extends View> T findViewWithTag(Object t) { return (T) findWithTag(t); }
    View findWithTag(Object t) { return t != null && t.equals(tag) ? this : null; }
    View findViewTraversal(int id) { return id == mId ? this : null; }
    View findByUid(int uid) { return uid == idoridUid ? this : null; }
    public final ViewParent getParent() { return mParent; }
    public View getRootView() { View v = this; while (v.mParent instanceof View) v = (View) v.mParent; return v; }
    public ViewGroup.LayoutParams getLayoutParams() { return mLayoutParams; }
    public void setLayoutParams(ViewGroup.LayoutParams p) { mLayoutParams = p; }

    public void setOnClickListener(OnClickListener l) { if (l != null) clickable = true; mOnClickListener = l; }
    public boolean hasOnClickListeners() { return mOnClickListener != null; }
    public void setOnLongClickListener(OnLongClickListener l) { longClick = l; }
    public void setOnTouchListener(OnTouchListener l) {}
    public void setOnFocusChangeListener(OnFocusChangeListener l) { focusListener = l; }
    public OnFocusChangeListener getOnFocusChangeListener() { return focusListener; }
    public void setOnKeyListener(OnKeyListener l) {}
    public void addOnLayoutChangeListener(OnLayoutChangeListener l) {}
    public void removeOnLayoutChangeListener(OnLayoutChangeListener l) {}
    public void addOnAttachStateChangeListener(OnAttachStateChangeListener l) {}
    public boolean performClick() {
        if (mOnClickListener != null) { mOnClickListener.onClick(this); return true; }
        return false;
    }
    public boolean callOnClick() { return performClick(); }
    public boolean performLongClick() { return longClick != null && longClick.onLongClick(this); }
    public void setVisibility(int v) { visibility = v; }
    public int getVisibility() { return visibility; }
    public boolean isShown() { return visibility == VISIBLE; }
    public void setEnabled(boolean e) { enabled = e; }
    public boolean isEnabled() { return enabled; }
    public void setClickable(boolean c) { clickable = c; }
    public boolean isClickable() { return clickable; }
    public void setLongClickable(boolean c) {}
    public void setSelected(boolean s) { selected = s; }
    public boolean isSelected() { return selected; }
    public void setActivated(boolean a) {}
    public void setFocusable(boolean f) { focusable = f; }
    public void setFocusableInTouchMode(boolean f) {}
    public boolean isFocusable() { return focusable; }
    public boolean requestFocus() { return true; }
    public void clearFocus() {}
    public boolean hasFocus() { return false; }
    public boolean isFocused() { return false; }
    public void setPadding(int l, int t, int r, int b) { pl = l; pt = t; pr = r; pb = b; }
    public void setPaddingRelative(int s, int t, int e, int b) { setPadding(s, t, e, b); }
    public int getPaddingLeft() { return pl; }
    public int getPaddingTop() { return pt; }
    public int getPaddingRight() { return pr; }
    public int getPaddingBottom() { return pb; }
    public int getPaddingStart() { return pl; }
    public int getPaddingEnd() { return pr; }
    public void setBackgroundColor(int c) { bgColor = c; bgRes = 0; }
    public void setBackgroundResource(int id) {
        bgRes = id; bgColor = 0;
        if (id != 0) { try { bgColor = getResources().getColor(id); bgRes = 0; } catch (RuntimeException e) {} }
    }
    public void setBackground(Drawable d) { bgColor = d == null ? 0 : d.idoridColor(); cornerRadius = d instanceof android.graphics.drawable.GradientDrawable ? ((android.graphics.drawable.GradientDrawable) d).idoridRadius() : 0; }
    public void setBackgroundDrawable(Drawable d) { setBackground(d); }
    public Drawable getBackground() { return bgColor != 0 ? new ColorDrawable(bgColor) : null; }
    public void setAlpha(float a) { alpha = a; }
    public float getAlpha() { return alpha; }
    public void setElevation(float e) {}
    public void setTranslationX(float x) {}
    public void setTranslationY(float y) {}
    public void setRotation(float r) {}
    public void setScaleX(float s) {}
    public void setScaleY(float s) {}
    public void setMinimumWidth(int w) { minW = w; }
    public void setMinimumHeight(int h) { minH = h; }
    public void setTag(Object t) { tag = t; }
    public Object getTag() { return tag; }
    public void setTag(int key, Object t) { if (keyedTags == null) keyedTags = new android.util.SparseArray<Object>(); keyedTags.put(key, t); }
    public Object getTag(int key) { return keyedTags == null ? null : keyedTags.get(key); }
    public void setContentDescription(CharSequence c) { contentDescription = c; }
    public CharSequence getContentDescription() { return contentDescription; }
    public boolean post(Runnable r) { return HANDLER.post(r); }
    public boolean postDelayed(Runnable r, long ms) { return HANDLER.postDelayed(r, ms); }
    public boolean removeCallbacks(Runnable r) { HANDLER.removeCallbacks(r); return true; }
    public Handler getHandler() { return HANDLER; }
    public void invalidate() {}
    public void postInvalidate() {}
    public void requestLayout() {}
    public void forceLayout() {}
    public int getWidth() { return 0; }
    public int getHeight() { return 0; }
    public int getMeasuredWidth() { return 0; }
    public int getMeasuredHeight() { return 0; }
    public int getLeft() { return 0; }
    public int getTop() { return 0; }
    public float getX() { return 0; }
    public float getY() { return 0; }
    public ViewTreeObserver getViewTreeObserver() { return new ViewTreeObserver(); }
    public boolean isAttachedToWindow() { return mParent != null; }
    public android.os.IBinder getWindowToken() { return null; }
    public android.os.IBinder getApplicationWindowToken() { return null; }
    public void setSoundEffectsEnabled(boolean b) {}
    public void setHapticFeedbackEnabled(boolean b) {}
    public boolean performHapticFeedback(int c) { return false; }
    public void setKeepScreenOn(boolean b) {}
    public void setTextAlignment(int a) {}
    public void setLayoutDirection(int d) {}
    public void setScrollContainer(boolean b) {}
    public void setOverScrollMode(int m) {}
    public void setVerticalScrollBarEnabled(boolean b) {}
    public void setHorizontalScrollBarEnabled(boolean b) {}
    public void scrollTo(int x, int y) {}
    public void scrollBy(int x, int y) {}
    public void setSystemUiVisibility(int v) {}
    public View getFocusedChild() { return null; }
    public ViewPropertyAnimator animate() { return new ViewPropertyAnimator(this); }
    public void startAnimation(Object a) {}
    public void clearAnimation() {}

    /** idorid: host type name. */
    protected String idoridType() { return "View"; }
    /** idorid: append type-specific JSON fields (each starting with ','). */
    protected void idoridProps(StringBuilder sb) {}

    public final void idoridJson(StringBuilder sb) {
        sb.append("{\"id\":").append(idoridUid);
        Json.key(sb, "type"); Json.str(sb, idoridType());
        if (visibility != VISIBLE) { Json.key(sb, "visible"); sb.append(visibility == INVISIBLE ? "\"invisible\"" : "false"); }
        if (pl != 0 || pt != 0 || pr != 0 || pb != 0) { Json.key(sb, "padding"); sb.append('[').append(pl).append(',').append(pt).append(',').append(pr).append(',').append(pb).append(']'); }
        if (bgColor != 0) { Json.key(sb, "bg"); Json.str(sb, android.graphics.Color.toHex(bgColor)); }
        if (cornerRadius != 0) { Json.key(sb, "radius"); Json.num(sb, cornerRadius); }
        if (alpha != 1f) { Json.key(sb, "alpha"); Json.num(sb, alpha); }
        if (!enabled) { Json.key(sb, "enabled"); sb.append("false"); }
        if (mOnClickListener != null || clickable) { Json.key(sb, "clickable"); sb.append("true"); }
        if (minW > 0) { Json.key(sb, "minWidth"); sb.append(minW); }
        if (minH > 0) { Json.key(sb, "minHeight"); sb.append(minH); }
        ViewGroup.LayoutParams lp = mLayoutParams;
        if (lp != null) {
            Json.key(sb, "width"); dim(sb, lp.width);
            Json.key(sb, "height"); dim(sb, lp.height);
            if (lp instanceof ViewGroup.MarginLayoutParams) {
                ViewGroup.MarginLayoutParams m = (ViewGroup.MarginLayoutParams) lp;
                if (m.leftMargin != 0 || m.topMargin != 0 || m.rightMargin != 0 || m.bottomMargin != 0) {
                    Json.key(sb, "margin"); sb.append('[').append(m.leftMargin).append(',').append(m.topMargin).append(',').append(m.rightMargin).append(',').append(m.bottomMargin).append(']');
                }
            }
            if (lp instanceof android.widget.LinearLayout.LayoutParams) {
                android.widget.LinearLayout.LayoutParams l = (android.widget.LinearLayout.LayoutParams) lp;
                if (l.weight != 0) { Json.key(sb, "weight"); Json.num(sb, l.weight); }
                String g = Gravity.describe(l.gravity);
                if (g != null) { Json.key(sb, "layoutGravity"); Json.str(sb, g); }
            } else if (lp instanceof android.widget.FrameLayout.LayoutParams) {
                String g = Gravity.describe(((android.widget.FrameLayout.LayoutParams) lp).gravity);
                if (g != null) { Json.key(sb, "layoutGravity"); Json.str(sb, g); }
            }
        }
        idoridProps(sb);
        sb.append('}');
    }
    private static void dim(StringBuilder sb, int d) {
        if (d == ViewGroup.LayoutParams.MATCH_PARENT) sb.append("\"match\"");
        else if (d == ViewGroup.LayoutParams.WRAP_CONTENT) sb.append("\"wrap\"");
        else sb.append(d);
    }
}
