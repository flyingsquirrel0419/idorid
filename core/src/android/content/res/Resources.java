package android.content.res;

import android.graphics.drawable.ColorDrawable;
import android.graphics.drawable.Drawable;
import android.util.AttributeSet;
import android.util.DisplayMetrics;
import android.util.TypedValue;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.Map;

/** Values come from the APK's resources.arsc (and the framework's) through VM natives. */
public class Resources {
    public static final int ID_NULL = 0;
    public static class NotFoundException extends RuntimeException {
        public NotFoundException() {}
        public NotFoundException(String m) { super(m); }
        public NotFoundException(String m, Exception cause) { super(m, cause); }
    }

    /** A set of style items applied in order; later non-forced styles do not override. */
    public final class Theme {
        final LinkedHashMap<Integer, String> attrs = new LinkedHashMap<Integer, String>();
        private int key;
        public void applyStyle(int resId, boolean force) {
            if (resId == 0) return;
            String[] items = nativeStyle(resId);
            if (items == null) return;
            for (int i = 0; i + 1 < items.length; i += 2) {
                Integer attr = Integer.valueOf(Integer.parseInt(items[i]));
                if (force || !attrs.containsKey(attr)) attrs.put(attr, items[i + 1]);
            }
            key = key * 31 + resId;
        }
        public void setTo(Theme other) { attrs.clear(); attrs.putAll(other.attrs); key = other.key; }
        public void rebase() {}
        public Resources getResources() { return Resources.this; }
        public int getChangingConfigurations() { return 0; }
        public void dump(int priority, String tag, String prefix) {}
        public Drawable getDrawable(int id) { return Resources.this.getDrawable(id, this); }
        public int[] getAttributeResolutionStack(int defStyleAttr, int defStyleRes, int explicitStyleRes) { return new int[0]; }
        public int getExplicitStyle(AttributeSet set) { return set == null ? 0 : set.getStyleAttribute(); }

        /** Theme attribute value with ?attr chains followed (null if unset). */
        public String idoridAttr(int attr) {
            String v = attrs.get(Integer.valueOf(attr));
            for (int guard = 0; guard < 16 && v != null && VMValue.kind(v) == 'a'; guard++) v = attrs.get(Integer.valueOf(VMValue.intOf(v, 0)));
            return v != null && VMValue.kind(v) == 'a' ? null : v;
        }
        public boolean resolveAttribute(int resid, TypedValue out, boolean resolveRefs) {
            String v = idoridAttr(resid);
            if (v == null) return false;
            fillValue(v, out, resolveRefs);
            return true;
        }
        public TypedArray obtainStyledAttributes(int[] wanted) { return obtainStyledAttributes(null, wanted, 0, 0); }
        public TypedArray obtainStyledAttributes(int resId, int[] wanted) throws NotFoundException {
            String[] values = new String[wanted.length];
            Map<Integer, String> style = styleMap(resId);
            for (int i = 0; i < wanted.length; i++) values[i] = resolveRef(style.get(Integer.valueOf(wanted[i])));
            return new TypedArray(Resources.this, this, wanted, values);
        }
        public TypedArray obtainStyledAttributes(AttributeSet set, int[] wanted, int defStyleAttr, int defStyleRes) {
            String[] values = new String[wanted.length];
            Map<Integer, String> xml = new HashMap<Integer, String>();
            int explicitStyle = 0;
            if (set instanceof android.view.XmlAttrs) {
                android.view.XmlAttrs x = (android.view.XmlAttrs) set;
                for (int i = 0; i < x.getAttributeCount(); i++) {
                    int id = x.getAttributeNameResource(i);
                    if (id != 0) xml.put(Integer.valueOf(id), x.idoridRawValue(i));
                }
                explicitStyle = x.getStyleAttribute();
            }
            Map<Integer, String> styleAttr = explicitStyle != 0 ? styleMap(explicitStyle) : null;
            Map<Integer, String> defAttr = null;
            if (defStyleAttr != 0) {
                String ref = idoridAttr(defStyleAttr);
                if (ref != null && VMValue.kind(ref) == 'r') defAttr = styleMap(VMValue.intOf(ref, 0));
            }
            Map<Integer, String> defRes = defAttr == null && defStyleRes != 0 ? styleMap(defStyleRes) : null;
            for (int i = 0; i < wanted.length; i++) {
                Integer a = Integer.valueOf(wanted[i]);
                String v = xml.get(a);
                if (v == null && styleAttr != null) v = styleAttr.get(a);
                if (v == null && defAttr != null) v = defAttr.get(a);
                if (v == null && defRes != null) v = defRes.get(a);
                if (v == null) v = attrs.get(a);
                values[i] = resolveRef(v);
            }
            return new TypedArray(Resources.this, this, wanted, values);
        }
        /** Follows ?attr through this theme; "@null"-style empties become null. */
        String resolveRef(String v) {
            for (int guard = 0; guard < 16 && v != null && VMValue.kind(v) == 'a'; guard++) v = attrs.get(Integer.valueOf(VMValue.intOf(v, 0)));
            if (v != null && VMValue.kind(v) == 'a') return null;
            return v;
        }
        public int hashCode() { return key; }
    }

    private static Resources system;
    private final DisplayMetrics metrics = new DisplayMetrics();
    private final Configuration config = new Configuration();
    private final HashMap<Integer, Map<Integer, String>> styleCache = new HashMap<Integer, Map<Integer, String>>();
    public Resources() {}
    public Resources(AssetManager assets, DisplayMetrics metrics, Configuration config) {}
    public static Resources getSystem() { if (system == null) system = new Resources(); return system; }

    static native String nativeString(int id);
    static native String nativeLayout(int id);
    static native String nativeName(int id);
    static native int nativeInt(int id, int kind);
    static native float nativeDimen(int id);
    static native int nativeIdentifier(String name, String type, String pkg);
    static native String[] nativeStringArray(int id);
    /** Style items (merged with parents) as [attrId, value, attrId, value, ...]; null if not a style. */
    static native String[] nativeStyle(int id);
    /** One resource value, encoded (see VMValue); null if missing. */
    static native String nativeValue(int id);
    /** A binary XML file from the APK, serialized like layouts. */
    static native String nativeXml(String path);

    Map<Integer, String> styleMap(int resId) {
        Integer k = Integer.valueOf(resId);
        Map<Integer, String> m = styleCache.get(k);
        if (m == null) {
            m = new HashMap<Integer, String>();
            String[] items = resId == 0 ? null : nativeStyle(resId);
            if (items != null) for (int i = 0; i + 1 < items.length; i += 2) m.put(Integer.valueOf(Integer.parseInt(items[i])), items[i + 1]);
            styleCache.put(k, m);
        }
        return m;
    }

    private static NotFoundException nf(int id) { return new NotFoundException("Resource ID #0x" + Integer.toHexString(id)); }

    /** idorid: encoded value of a resource, following references to a primitive or file. */
    public String idoridValue(int id) { return id == 0 ? null : nativeValue(id); }

    void fillValue(String v, TypedValue out, boolean resolveRefs) {
        char k = VMValue.kind(v);
        if (k == 'r' && resolveRefs) {
            int id = VMValue.intOf(v, 0);
            String r = idoridValue(id);
            if (r != null && VMValue.kind(r) != 'r') { fillValue(r, out, false); out.resourceId = id; return; }
        }
        out.string = null; out.resourceId = 0; out.data = 0; out.assetCookie = 0;
        switch (k) {
            case 'r': out.type = 1; out.data = VMValue.intOf(v, 0); out.resourceId = out.data; break;
            case 'a': out.type = 2; out.data = VMValue.intOf(v, 0); break;
            case 's': out.type = 3; out.string = VMValue.body(v); break;
            case 'p': out.type = 3; out.string = VMValue.body(v); break;
            case 'f': out.type = 4; out.data = Float.floatToIntBits(VMValue.floatOf(v, 0)); break;
            case 'd': out.type = 5; out.data = complexDimension(VMValue.dimension(v, 0)); break;
            case 'b': out.type = 18; out.data = VMValue.boolOf(v, false) ? -1 : 0; break;
            case 'c': out.type = 28; out.data = VMValue.intOf(v, 0); break;
            default: out.type = 16; out.data = VMValue.intOf(v, 0);
        }
    }
    /** Encodes a dp value as a TypedValue complex dimension (unit dp, radix 23p0 or 16p7). */
    static int complexDimension(float dp) {
        if (dp == Math.round(dp) && Math.abs(dp) < (1 << 23)) return (Math.round(dp) << 8) | 1;
        int mantissa = Math.round(dp * 128);
        return (mantissa << 8) | (1 << 4) | 1;
    }

    public String getString(int id) { String s = nativeString(id); if (s == null) throw nf(id); return s; }
    public String getString(int id, Object... args) { return String.format(getString(id), args); }
    public CharSequence getText(int id) { return getString(id); }
    public CharSequence getText(int id, CharSequence def) { String s = nativeString(id); return s == null ? def : s; }
    public String[] getStringArray(int id) { String[] a = nativeStringArray(id); if (a == null) throw nf(id); return a; }
    public CharSequence[] getTextArray(int id) { return getStringArray(id); }
    public int[] getIntArray(int id) { String[] a = getStringArray(id); int[] r = new int[a.length]; for (int i = 0; i < a.length; i++) { try { r[i] = Integer.parseInt(a[i]); } catch (RuntimeException e) {} } return r; }
    public TypedArray obtainTypedArray(int id) {
        String[] a = getStringArray(id);
        int[] idx = new int[a.length];
        String[] v = new String[a.length];
        for (int i = 0; i < a.length; i++) { idx[i] = i; v[i] = "s:" + a[i]; }
        return new TypedArray(this, null, idx, v);
    }
    public String getQuantityString(int id, int q) { return getString(id); }
    public String getQuantityString(int id, int q, Object... args) { return String.format(getString(id), args); }
    public CharSequence getQuantityText(int id, int q) { return getString(id); }
    public int getColor(int id) { return getColor(id, null); }
    public int getColor(int id, Theme t) {
        String v = idoridValue(id);
        if (v == null) throw nf(id);
        if (VMValue.kind(v) == 'p') { ColorStateList c = ColorStateList.idoridInflate(this, VMValue.body(v), t); return c == null ? 0 : c.getDefaultColor(); }
        return VMValue.intOf(v, 0);
    }
    public ColorStateList getColorStateList(int id) { return getColorStateList(id, null); }
    public ColorStateList getColorStateList(int id, Theme t) {
        String v = idoridValue(id);
        if (v == null) throw nf(id);
        if (VMValue.kind(v) == 'p') { ColorStateList c = ColorStateList.idoridInflate(this, VMValue.body(v), t); if (c != null) return c; }
        return ColorStateList.valueOf(VMValue.intOf(v, 0));
    }
    public int getInteger(int id) { String v = idoridValue(id); if (v == null) throw nf(id); return VMValue.intOf(v, 0); }
    public boolean getBoolean(int id) { String v = idoridValue(id); if (v == null) throw nf(id); return VMValue.boolOf(v, false); }
    public float getFloat(int id) { String v = idoridValue(id); if (v == null) throw nf(id); return VMValue.floatOf(v, 0); }
    public float getFraction(int id, int base, int pbase) { String v = idoridValue(id); if (v == null) throw nf(id); return VMValue.floatOf(v, 0) * base; }
    public float getDimension(int id) { String v = idoridValue(id); if (v == null) throw nf(id); return VMValue.dimension(v, 0); }
    public int getDimensionPixelSize(int id) { float f = getDimension(id); int r = (int) (f >= 0 ? f + 0.5f : f - 0.5f); return r != 0 ? r : f == 0 ? 0 : f > 0 ? 1 : -1; }
    public int getDimensionPixelOffset(int id) { return (int) getDimension(id); }
    public Drawable getDrawable(int id) { return getDrawable(id, null); }
    public Drawable getDrawable(int id, Theme t) {
        String v = idoridValue(id);
        if (v == null) throw nf(id);
        return drawableFor(id, v, t);
    }
    public Drawable getDrawableForDensity(int id, int density) { return getDrawable(id); }
    public Drawable getDrawableForDensity(int id, int density, Theme t) { return getDrawable(id, t); }
    /** idorid: drawable for an encoded value (color, bitmap file, or drawable XML). */
    public Drawable drawableFor(int id, String v, Theme t) {
        switch (VMValue.kind(v)) {
            case 'c': case 'i': return new ColorDrawable(VMValue.intOf(v, 0));
            case 'p': {
                String path = VMValue.body(v);
                if (path.endsWith(".xml")) { Drawable d = VMDrawables.inflate(this, path, t); if (d != null) return d; return new ColorDrawable(0); }
                return Drawable.fromResource(id);
            }
            default: return Drawable.fromResource(id);
        }
    }
    public void getValue(int id, TypedValue out, boolean resolveRefs) {
        String v = idoridValue(id);
        if (v == null) throw nf(id);
        fillValue(v, out, resolveRefs);
        if (out.resourceId == 0) out.resourceId = id;
    }
    public void getValue(String name, TypedValue out, boolean resolveRefs) { getValue(getIdentifier(name, null, null), out, resolveRefs); }
    public void getValueForDensity(int id, int density, TypedValue out, boolean resolveRefs) { getValue(id, out, resolveRefs); }
    public TypedArray obtainAttributes(AttributeSet set, int[] attrs) { return newTheme().obtainStyledAttributes(set, attrs, 0, 0); }
    public String getResourceName(int id) { String n = nativeName(id); if (n == null) throw nf(id); return n; }
    public String getResourceEntryName(int id) { String n = getResourceName(id); return n.substring(n.lastIndexOf('/') + 1); }
    public String getResourceTypeName(int id) { String n = getResourceName(id); return n.substring(n.indexOf(':') + 1, n.indexOf('/')); }
    public String getResourcePackageName(int id) { String n = getResourceName(id); int c = n.indexOf(':'); return c < 0 ? "" : n.substring(0, c); }
    public int getIdentifier(String name, String type, String pkg) { return nativeIdentifier(name, type, pkg); }
    public DisplayMetrics getDisplayMetrics() { return metrics; }
    public Configuration getConfiguration() { return config; }
    public void updateConfiguration(Configuration c, DisplayMetrics m) { if (c != null) config.setTo(c); }
    public final Theme newTheme() { return new Theme(); }
    public final void flushLayoutCache() {}
    public final AssetManager getAssets() { return new AssetManager(); }
    public java.io.InputStream openRawResource(int id) throws NotFoundException { throw nf(id); }
    public java.io.InputStream openRawResource(int id, TypedValue v) throws NotFoundException { throw nf(id); }
    public XmlResourceParser getLayout(int id) { throw new UnsupportedOperationException("idorid: getLayout parser"); }
    public XmlResourceParser getXml(int id) { throw new UnsupportedOperationException("idorid: getXml parser"); }
    public XmlResourceParser getAnimation(int id) { throw new UnsupportedOperationException("idorid: getAnimation parser"); }
    public android.graphics.Typeface getFont(int id) { return android.graphics.Typeface.DEFAULT; }
    /** idorid: serialized layout tree for LayoutInflater. */
    public String getLayoutSource(int id) { String s = nativeLayout(id); if (s == null) throw nf(id); return s; }
    /** idorid: serialized binary XML file (res/color, res/drawable, res/xml). */
    public String idoridXml(String path) { return nativeXml(path); }
}
