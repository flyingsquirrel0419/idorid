package android.content.res;

import android.util.DisplayMetrics;

/** Values come from the APK's resources.arsc through VM natives. */
public class Resources {
    public static class NotFoundException extends RuntimeException {
        public NotFoundException() {}
        public NotFoundException(String m) { super(m); }
    }
    public final class Theme {
        public void applyStyle(int resId, boolean force) {}
    }
    private static Resources system;
    private final DisplayMetrics metrics = new DisplayMetrics();
    private final Configuration config = new Configuration();
    public Resources() {}
    public static Resources getSystem() { if (system == null) system = new Resources(); return system; }

    static native String nativeString(int id);
    static native String nativeLayout(int id);
    static native String nativeName(int id);
    static native int nativeInt(int id, int kind);
    static native float nativeDimen(int id);
    static native int nativeIdentifier(String name, String type, String pkg);
    static native String[] nativeStringArray(int id);

    private static NotFoundException nf(int id) { return new NotFoundException("Resource ID #0x" + Integer.toHexString(id)); }

    public String getString(int id) { String s = nativeString(id); if (s == null) throw nf(id); return s; }
    public String getString(int id, Object... args) { return String.format(getString(id), args); }
    public CharSequence getText(int id) { return getString(id); }
    public CharSequence getText(int id, CharSequence def) { String s = nativeString(id); return s == null ? def : s; }
    public String[] getStringArray(int id) { String[] a = nativeStringArray(id); if (a == null) throw nf(id); return a; }
    public CharSequence[] getTextArray(int id) { return getStringArray(id); }
    public String getQuantityString(int id, int q) { return getString(id); }
    public String getQuantityString(int id, int q, Object... args) { return String.format(getString(id), args); }
    public int getColor(int id) { return nativeInt(id, 0); }
    public int getColor(int id, Theme t) { return nativeInt(id, 0); }
    public int getInteger(int id) { return nativeInt(id, 1); }
    public boolean getBoolean(int id) { return nativeInt(id, 2) != 0; }
    public float getDimension(int id) { return nativeDimen(id); }
    public int getDimensionPixelSize(int id) { return Math.round(nativeDimen(id)); }
    public int getDimensionPixelOffset(int id) { return (int) nativeDimen(id); }
    public String getResourceName(int id) { String n = nativeName(id); if (n == null) throw nf(id); return n; }
    public String getResourceEntryName(int id) { String n = getResourceName(id); return n.substring(n.lastIndexOf('/') + 1); }
    public String getResourceTypeName(int id) { String n = getResourceName(id); return n.substring(n.indexOf(':') + 1, n.indexOf('/')); }
    public int getIdentifier(String name, String type, String pkg) { return nativeIdentifier(name, type, pkg); }
    public DisplayMetrics getDisplayMetrics() { return metrics; }
    public Configuration getConfiguration() { return config; }
    public final Theme newTheme() { return new Theme(); }
    /** idorid: serialized layout tree for LayoutInflater. */
    public String getLayoutSource(int id) { String s = nativeLayout(id); if (s == null) throw nf(id); return s; }
}
