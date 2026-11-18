package android.view;

import android.content.Context;
import java.util.ArrayList;

/**
 * Inflates the VM-serialized layout tree. Element: \u0001tag, attribute:
 * \u0002name\u0004value, end of element: \u0003.
 */
public class LayoutInflater {
    private final Context ctx;
    protected LayoutInflater(Context c) { ctx = c; }
    public static LayoutInflater from(Context c) { return new LayoutInflater(c); }
    public Context getContext() { return ctx; }
    public LayoutInflater cloneInContext(Context c) { return new LayoutInflater(c); }
    public View inflate(int resource, ViewGroup root) { return inflate(resource, root, root != null); }
    public View inflate(int resource, ViewGroup root, boolean attach) {
        String src = ctx.getResources().getLayoutSource(resource);
        int[] pos = { 0 };
        View v = parse(src, pos, root, attach, true);
        return attach && root != null ? root : v;
    }

    private View parse(String s, int[] pos, ViewGroup parent, boolean attach, boolean top) {
        int p = pos[0];
        if (s.charAt(p) != '\u0001') throw new IllegalStateException("bad layout");
        int e = next(s, p + 1);
        String tag = s.substring(p + 1, e);
        p = e;
        ArrayList<String> names = new ArrayList<String>(), vals = new ArrayList<String>();
        while (p < s.length() && s.charAt(p) == '\u0002') {
            int sep = s.indexOf('\u0004', p);
            int end = next(s, sep + 1);
            names.add(s.substring(p + 1, sep));
            vals.add(s.substring(sep + 1, end));
            p = end;
        }
        XmlAttrs attrs = new XmlAttrs(names.toArray(new String[0]), vals.toArray(new String[0]));
        pos[0] = p;
        if (tag.equals("merge")) {
            if (parent == null) throw new InflateException("<merge /> can be used only with a valid ViewGroup root and attachToRoot=true");
            parseChildren(s, pos, parent);
            pos[0]++;
            return parent;
        }
        if (tag.equals("include")) {
            int layout = attrs.integer("layout", 0);
            while (s.charAt(pos[0]) != '\u0003') parse(s, pos, null, false, false);
            pos[0]++;
            View v = inflate(layout, null, false);
            int id = attrs.integer("id", View.NO_ID);
            if (id != View.NO_ID) v.setId(id);
            if (parent != null) {
                ViewGroup.LayoutParams lp = attrs.has("layout_width") ? parent.generateLayoutParams(attrs) : v.getLayoutParams();
                parent.addView(v, lp == null ? parent.generateDefaultLayoutParams() : lp);
            }
            return v;
        }
        View v = createView(tag, attrs);
        if (parent != null) {
            ViewGroup.LayoutParams lp = parent.generateLayoutParams(attrs);
            if (!top || attach) parent.addView(v, lp);
            else v.setLayoutParams(lp);
        } else if (attrs.has("layout_width")) {
            v.setLayoutParams(new ViewGroup.MarginLayoutParams(ctx, attrs));
        }
        if (v instanceof ViewGroup) parseChildren(s, pos, (ViewGroup) v);
        else while (s.charAt(pos[0]) != '\u0003') parse(s, pos, null, false, false);
        pos[0]++; // \u0003
        if (v instanceof ViewGroup) ((ViewGroup) v).onFinishInflate();
        return v;
    }
    private void parseChildren(String s, int[] pos, ViewGroup parent) {
        while (s.charAt(pos[0]) != '\u0003') parse(s, pos, parent, true, false);
    }
    private static int next(String s, int from) {
        int i = from;
        while (i < s.length()) { char c = s.charAt(i); if (c <= '\u0004') break; i++; }
        return i;
    }
    protected View createView(String tag, XmlAttrs attrs) {
        String[] candidates;
        if (tag.equals("view")) tag = attrs.string("class");
        if (tag.indexOf('.') >= 0) candidates = new String[] { mapAndroidx(tag) };
        else candidates = new String[] { "android.widget." + tag, "android.view." + tag, "android.webkit." + tag };
        for (String name : candidates) {
            Class<?> c;
            try { c = Class.forName(name); } catch (ClassNotFoundException e) { continue; }
            return (View) VMInflate.construct(c, ctx, attrs);
        }
        android.util.Log.w("idorid", "Unsupported view <" + tag + ">, using FrameLayout");
        return new android.widget.FrameLayout(ctx, attrs);
    }
    /** Common AndroidX widgets map onto framework equivalents when the app doesn't bundle them. */
    private static String mapAndroidx(String tag) {
        try { Class.forName(tag); return tag; } catch (ClassNotFoundException e) {}
        if (tag.endsWith(".AppCompatTextView") || tag.endsWith(".MaterialTextView")) return "android.widget.TextView";
        if (tag.endsWith(".AppCompatButton") || tag.endsWith(".MaterialButton")) return "android.widget.Button";
        if (tag.endsWith(".AppCompatEditText") || tag.endsWith(".TextInputEditText")) return "android.widget.EditText";
        if (tag.endsWith(".AppCompatImageView")) return "android.widget.ImageView";
        if (tag.endsWith(".NestedScrollView")) return "android.widget.ScrollView";
        if (tag.endsWith(".LinearLayoutCompat")) return "android.widget.LinearLayout";
        if (tag.endsWith(".ConstraintLayout") || tag.endsWith(".CoordinatorLayout") || tag.endsWith(".CardView") || tag.endsWith(".MaterialCardView")) return "android.widget.FrameLayout";
        return tag;
    }
    public interface Factory { View onCreateView(String name, Context context, android.util.AttributeSet attrs); }
    public interface Factory2 extends Factory { View onCreateView(View parent, String name, Context context, android.util.AttributeSet attrs); }
    public void setFactory(Factory f) {}
    public void setFactory2(Factory2 f) {}
}
