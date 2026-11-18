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
    private OnTouchListener touchListener;
    private static boolean dirty;
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
    public void setLayoutParams(ViewGroup.LayoutParams p) { if (p == null) throw new NullPointerException("Layout parameters cannot be null"); mLayoutParams = p; requestLayout(); }

    public void setOnClickListener(OnClickListener l) { if (l != null) clickable = true; mOnClickListener = l; }
    public boolean hasOnClickListeners() { return mOnClickListener != null; }
    public void setOnLongClickListener(OnLongClickListener l) { longClick = l; }
    public void setOnTouchListener(OnTouchListener l) { touchListener = l; }
    public boolean dispatchTouchEvent(MotionEvent e) { return enabled && ((touchListener != null && touchListener.onTouch(this,e)) || onTouchEvent(e)); }
    public boolean onTouchEvent(MotionEvent e) { if (!enabled || !clickable) return false; if (e.getActionMasked() == MotionEvent.ACTION_UP) performClick(); return true; }
    public void setOnFocusChangeListener(OnFocusChangeListener l) { focusListener = l; }
    public OnFocusChangeListener getOnFocusChangeListener() { return focusListener; }
    public void setOnKeyListener(OnKeyListener l) {}
    public void addOnAttachStateChangeListener(OnAttachStateChangeListener l) {}
    public boolean performClick() {
        if (mOnClickListener != null) { mOnClickListener.onClick(this); return true; }
        return false;
    }
    public boolean callOnClick() { return performClick(); }
    public boolean performLongClick() { return longClick != null && longClick.onLongClick(this); }
    public void setVisibility(int v) { if (v != visibility) { boolean gone = v == GONE || visibility == GONE; visibility = v; if (gone) requestLayout(); } }
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
    public void setPadding(int l, int t, int r, int b) { if (l != pl || t != pt || r != pr || b != pb) { pl = l; pt = t; pr = r; pb = b; requestLayout(); } }
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
    public void setMinimumWidth(int w) { minW = w; requestLayout(); }
    public void setMinimumHeight(int h) { minH = h; requestLayout(); }
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
    public void invalidate() { if (VMViews.overridesOnDraw(this)) dirty = true; }
    public void postInvalidate() { invalidate(); }
    public void postInvalidateOnAnimation() { invalidate(); }
    public void postOnAnimation(Runnable r) { postDelayed(r,16); }
    public static boolean idoridTakeDirty() { boolean d=dirty; dirty=false; return d; }
    protected void onSizeChanged(int w,int h,int oldw,int oldh) {}
    protected void onDraw(android.graphics.Canvas c) {}
    public void draw(android.graphics.Canvas c) { onDraw(c); }
    /** idorid: host-reported size (legacy path; layout normally sets the frame). */
    public void idoridSetSize(int w,int h) { if (w != getWidth() || h != getHeight()) setFrame(mLeft, mTop, mLeft + w, mTop + h); }

    // ---------------------------------------------------------------- measure/layout
    public static class MeasureSpec {
        private static final int MODE_SHIFT = 30, MODE_MASK = 3 << MODE_SHIFT;
        public static final int UNSPECIFIED = 0, EXACTLY = 1 << MODE_SHIFT, AT_MOST = 2 << MODE_SHIFT;
        public static int makeMeasureSpec(int size, int mode) { return (size & ~MODE_MASK) | (mode & MODE_MASK); }
        public static int makeSafeMeasureSpec(int size, int mode) { return makeMeasureSpec(size, mode); }
        public static int getMode(int spec) { return spec & MODE_MASK; }
        public static int getSize(int spec) { return spec & ~MODE_MASK; }
        public static String toString(int spec) {
            int m = getMode(spec);
            return "MeasureSpec: " + (m == EXACTLY ? "EXACTLY " : m == AT_MOST ? "AT_MOST " : "UNSPECIFIED ") + getSize(spec);
        }
    }
    public static final int MEASURED_SIZE_MASK = 0x00ffffff, MEASURED_STATE_MASK = 0xff000000, MEASURED_HEIGHT_STATE_SHIFT = 16, MEASURED_STATE_TOO_SMALL = 0x01000000;

    int mLeft, mTop, mRight, mBottom;
    private int mMeasuredWidth, mMeasuredHeight;
    private int mOldWidthSpec = Integer.MIN_VALUE, mOldHeightSpec = Integer.MIN_VALUE;
    boolean mForceLayout = true, mLayoutRequired = true;
    private boolean measuredCalled;
    int mScrollX, mScrollY;
    private float translationX, translationY, scaleX = 1, scaleY = 1, rotation;
    private java.util.ArrayList<OnLayoutChangeListener> layoutListeners;
    private static boolean sLayoutRequested = true;
    /** idorid: true once since the last call when any view asked for layout. */
    public static boolean idoridTakeLayoutRequest() { boolean r = sLayoutRequested; sLayoutRequested = false; return r; }

    public final void measure(int widthMeasureSpec, int heightMeasureSpec) {
        if (mForceLayout || widthMeasureSpec != mOldWidthSpec || heightMeasureSpec != mOldHeightSpec) {
            measuredCalled = false;
            onMeasure(widthMeasureSpec, heightMeasureSpec);
            if (!measuredCalled) throw new IllegalStateException("View with id " + getId() + ": " + getClass().getName() + "#onMeasure() did not set the measured dimension by calling setMeasuredDimension()");
            mOldWidthSpec = widthMeasureSpec;
            mOldHeightSpec = heightMeasureSpec;
            mLayoutRequired = true;
        }
    }
    protected void onMeasure(int widthMeasureSpec, int heightMeasureSpec) {
        setMeasuredDimension(getDefaultSize(getSuggestedMinimumWidth(), widthMeasureSpec), getDefaultSize(getSuggestedMinimumHeight(), heightMeasureSpec));
    }
    protected final void setMeasuredDimension(int w, int h) { mMeasuredWidth = w; mMeasuredHeight = h; measuredCalled = true; }
    public static int getDefaultSize(int size, int spec) {
        int mode = MeasureSpec.getMode(spec), s = MeasureSpec.getSize(spec);
        return mode == MeasureSpec.UNSPECIFIED ? size : s;
    }
    public static int resolveSize(int size, int spec) { return resolveSizeAndState(size, spec, 0) & MEASURED_SIZE_MASK; }
    public static int resolveSizeAndState(int size, int spec, int childState) {
        int mode = MeasureSpec.getMode(spec), s = MeasureSpec.getSize(spec);
        int result;
        if (mode == MeasureSpec.AT_MOST) result = s < size ? (s | MEASURED_STATE_TOO_SMALL) : size;
        else if (mode == MeasureSpec.EXACTLY) result = s;
        else result = size;
        return result | (childState & MEASURED_STATE_MASK);
    }
    public static int combineMeasuredStates(int a, int b) { return a | b; }
    protected int getSuggestedMinimumWidth() { return minW; }
    protected int getSuggestedMinimumHeight() { return minH; }
    public int getMinimumWidth() { return minW; }
    public int getMinimumHeight() { return minH; }
    public final int getMeasuredWidth() { return mMeasuredWidth & MEASURED_SIZE_MASK; }
    public final int getMeasuredHeight() { return mMeasuredHeight & MEASURED_SIZE_MASK; }
    public final int getMeasuredWidthAndState() { return mMeasuredWidth; }
    public final int getMeasuredHeightAndState() { return mMeasuredHeight; }
    public final int getMeasuredState() { return (mMeasuredWidth & MEASURED_STATE_MASK) | ((mMeasuredHeight >> MEASURED_HEIGHT_STATE_SHIFT) & (MEASURED_STATE_MASK >> MEASURED_HEIGHT_STATE_SHIFT)); }

    public void layout(int l, int t, int r, int b) {
        int ol = mLeft, ot = mTop, or = mRight, ob = mBottom;
        boolean changed = setFrame(l, t, r, b);
        if (changed || mLayoutRequired) {
            onLayout(changed, l, t, r, b);
            mLayoutRequired = false;
            if (layoutListeners != null) for (OnLayoutChangeListener li : new java.util.ArrayList<OnLayoutChangeListener>(layoutListeners)) li.onLayoutChange(this, l, t, r, b, ol, ot, or, ob);
        }
        mForceLayout = false;
    }
    protected void onLayout(boolean changed, int l, int t, int r, int b) {}
    protected boolean setFrame(int l, int t, int r, int b) {
        if (mLeft == l && mTop == t && mRight == r && mBottom == b) return false;
        int ow = mRight - mLeft, oh = mBottom - mTop, nw = r - l, nh = b - t;
        mLeft = l; mTop = t; mRight = r; mBottom = b;
        if (nw != ow || nh != oh) { onSizeChanged(nw, nh, ow, oh); invalidate(); }
        return true;
    }
    public void requestLayout() {
        mForceLayout = true;
        mLayoutRequired = true;
        sLayoutRequested = true;
        if (mParent instanceof View && !((View) mParent).mForceLayout) ((View) mParent).requestLayout();
    }
    public void forceLayout() { mForceLayout = true; mLayoutRequired = true; }
    public boolean isLayoutRequested() { return mForceLayout; }
    public boolean isInLayout() { return false; }
    public boolean isLaidOut() { return mRight > mLeft || mBottom > mTop; }
    public final int getWidth() { return mRight - mLeft; }
    public final int getHeight() { return mBottom - mTop; }
    public final int getLeft() { return mLeft; }
    public final int getTop() { return mTop; }
    public final int getRight() { return mRight; }
    public final int getBottom() { return mBottom; }
    public final void setLeft(int l) { setFrame(l, mTop, mRight, mBottom); }
    public final void setTop(int t) { setFrame(mLeft, t, mRight, mBottom); }
    public final void setRight(int r) { setFrame(mLeft, mTop, r, mBottom); }
    public final void setBottom(int b) { setFrame(mLeft, mTop, mRight, b); }
    public void offsetLeftAndRight(int d) { mLeft += d; mRight += d; }
    public void offsetTopAndBottom(int d) { mTop += d; mBottom += d; }
    public float getX() { return mLeft + translationX; }
    public float getY() { return mTop + translationY; }
    public void setX(float x) { translationX = x - mLeft; }
    public void setY(float y) { translationY = y - mTop; }
    public void setTranslationX(float x) { translationX = x; }
    public void setTranslationY(float y) { translationY = y; }
    public void setRotation(float r) { rotation = r; }
    public void setScaleX(float s) { scaleX = s; }
    public void setScaleY(float s) { scaleY = s; }
    public float getTranslationX() { return translationX; }
    public float getTranslationY() { return translationY; }
    public float getScaleX() { return scaleX; }
    public float getScaleY() { return scaleY; }
    public float getRotation() { return rotation; }
    public int getScrollX() { return mScrollX; }
    public int getScrollY() { return mScrollY; }
    public void scrollTo(int x, int y) { if (x != mScrollX || y != mScrollY) { int ox = mScrollX, oy = mScrollY; mScrollX = x; mScrollY = y; onScrollChanged(x, y, ox, oy); invalidate(); } }
    public void scrollBy(int x, int y) { scrollTo(mScrollX + x, mScrollY + y); }
    protected void onScrollChanged(int l, int t, int oldl, int oldt) {}
    public void addOnLayoutChangeListener(OnLayoutChangeListener l) { if (layoutListeners == null) layoutListeners = new java.util.ArrayList<OnLayoutChangeListener>(); layoutListeners.add(l); }
    public void removeOnLayoutChangeListener(OnLayoutChangeListener l) { if (layoutListeners != null) layoutListeners.remove(l); }
    public void getLocationOnScreen(int[] out) { getLocationInWindow(out); }
    public void getLocationInWindow(int[] out) {
        int x = 0, y = 0;
        View v = this;
        while (v != null) { x += v.mLeft - (v == this ? 0 : v.mScrollX); y += v.mTop - (v == this ? 0 : v.mScrollY); v = v.mParent instanceof View ? (View) v.mParent : null; }
        out[0] = x; out[1] = y;
    }
    public boolean getGlobalVisibleRect(android.graphics.Rect r) { int[] p = new int[2]; getLocationInWindow(p); r.set(p[0], p[1], p[0] + getWidth(), p[1] + getHeight()); return getWidth() > 0 && getHeight() > 0; }
    public boolean getLocalVisibleRect(android.graphics.Rect r) { r.set(0, 0, getWidth(), getHeight()); return getWidth() > 0 && getHeight() > 0; }
    public void getHitRect(android.graphics.Rect r) { r.set(mLeft, mTop, mRight, mBottom); }
    public void getDrawingRect(android.graphics.Rect r) { r.set(mScrollX, mScrollY, mScrollX + getWidth(), mScrollY + getHeight()); }
    public int getBaseline() { return -1; }
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
        if (touchListener != null || VMViews.overridesOnTouch(this)) { Json.key(sb, "touch"); sb.append("true"); }
        if (VMViews.overridesOnDraw(this)) { Json.key(sb, "draw"); android.graphics.Canvas c = new android.graphics.Canvas(getWidth(),getHeight()); if (visibility == VISIBLE && getWidth() > 0 && getHeight() > 0) draw(c); sb.append(c.idoridOps()); }
        idoridProps(sb);
        sb.append('}');
    }
    private static void dim(StringBuilder sb, int d) {
        if (d == ViewGroup.LayoutParams.MATCH_PARENT) sb.append("\"match\"");
        else if (d == ViewGroup.LayoutParams.WRAP_CONTENT) sb.append("\"wrap\"");
        else sb.append(d);
    }
}
