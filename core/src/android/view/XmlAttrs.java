package android.view;

import android.util.AttributeSet;

/**
 * idorid: attributes of one inflated XML element. Values arrive pre-resolved
 * from the VM with a type prefix: s: string, i: int, d: dimension ("16dp"),
 * c: color (int), r: resource reference (int), b: boolean, f: float.
 */
public final class XmlAttrs implements AttributeSet {
    final String[] names;
    final String[] values;
    XmlAttrs(String[] n, String[] v) { names = n; values = v; }
    public static final XmlAttrs EMPTY = new XmlAttrs(new String[0], new String[0]);

    int index(String name) { for (int i = 0; i < names.length; i++) if (names[i].equals(name)) return i; return -1; }
    public boolean has(String name) { return index(name) >= 0; }
    public String raw(String name) { int i = index(name); return i < 0 ? null : values[i]; }
    public String string(String name) {
        String v = raw(name);
        if (v == null) return null;
        return v.length() >= 2 && v.charAt(1) == ':' ? v.substring(2) : v;
    }
    public int integer(String name, int def) {
        String v = raw(name);
        if (v == null) return def;
        String b = v.substring(2);
        try {
            switch (v.charAt(0)) {
                case 'i': case 'c': case 'r': return Integer.parseInt(b);
                case 'b': return b.equals("true") ? 1 : 0;
                case 'f': return (int) Float.parseFloat(b);
                case 'd': return (int) dimension(name, def);
                default: return Integer.decode(b);
            }
        } catch (RuntimeException e) { return def; }
    }
    /** Dimension in idorid pixels (1px == 1dp == 1sp). match_parent/wrap_content arrive as i:-1 / i:-2. */
    public float dimension(String name, float def) {
        String v = raw(name);
        if (v == null) return def;
        String b = v.substring(2);
        if (v.charAt(0) == 'i') return Integer.parseInt(b);
        int u = b.length();
        while (u > 0 && Character.isLetter(b.charAt(u - 1))) u--;
        try { return Float.parseFloat(b.substring(0, u)); } catch (RuntimeException e) { return def; }
    }
    public boolean bool(String name, boolean def) {
        String v = string(name);
        return v == null ? def : v.equals("true") || v.equals("1");
    }
    public boolean isRef(String name) { String v = raw(name); return v != null && v.startsWith("r:"); }

    public int getAttributeCount() { return names.length; }
    public String getAttributeName(int i) { return names[i]; }
    public String getAttributeValue(int i) { String v = values[i]; return v.length() >= 2 ? v.substring(2) : v; }
    public String getAttributeValue(String ns, String name) { return string(name); }
    public String getPositionDescription() { return "idorid layout"; }
    public int getAttributeNameResource(int i) { return 0; }
    public int getAttributeListValue(String ns, String a, String[] opts, int d) {
        String v = string(a); if (v == null) return d;
        for (int i = 0; i < opts.length; i++) if (opts[i].equals(v)) return i;
        return d;
    }
    public boolean getAttributeBooleanValue(String ns, String a, boolean d) { return bool(a, d); }
    public int getAttributeResourceValue(String ns, String a, int d) { return isRef(a) ? integer(a, d) : d; }
    public int getAttributeIntValue(String ns, String a, int d) { return integer(a, d); }
    public int getAttributeUnsignedIntValue(String ns, String a, int d) { return integer(a, d); }
    public float getAttributeFloatValue(String ns, String a, float d) { return dimension(a, d); }
    public int getAttributeResourceValue(int i, int d) { return values[i].startsWith("r:") ? Integer.parseInt(values[i].substring(2)) : d; }
    public int getAttributeIntValue(int i, int d) { return integer(names[i], d); }
    public boolean getAttributeBooleanValue(int i, boolean d) { return bool(names[i], d); }
    public float getAttributeFloatValue(int i, float d) { return dimension(names[i], d); }
    public String getIdAttribute() { return string("id"); }
    public String getClassAttribute() { return string("class"); }
    public int getIdAttributeResourceValue(int d) { return integer("id", d); }
    public int getStyleAttribute() { return integer("style", 0); }
}
