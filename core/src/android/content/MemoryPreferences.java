package android.content;

import java.util.*;

/** In-memory SharedPreferences (persistence is a TODO in the host bridge). */
final class MemoryPreferences implements SharedPreferences {
    private static final HashMap<String, MemoryPreferences> ALL = new HashMap<String, MemoryPreferences>();
    private final HashMap<String, Object> map = new HashMap<String, Object>();
    private final ArrayList<OnSharedPreferenceChangeListener> listeners = new ArrayList<OnSharedPreferenceChangeListener>();
    private String name;
    static MemoryPreferences get(String name) {
        MemoryPreferences p = ALL.get(name);
        if (p == null) { p = new MemoryPreferences(); p.name = name; p.load(); ALL.put(name, p); }
        return p;
    }
    private String file() { return "shared_prefs/" + name + ".prefs"; }
    /** Format: one entry per line, "<type><key>\u0001<value>"; type s/i/l/f/b/S (set, values \u0002-separated). */
    private void load() {
        String s = VMStorage.load(file());
        if (s == null) return;
        for (String line : s.split("\n")) {
            int sep = line.indexOf('\u0001');
            if (sep < 1) continue;
            char t = line.charAt(0);
            String k = unesc(line.substring(1, sep)), v = line.substring(sep + 1);
            try {
                switch (t) {
                    case 's': map.put(k, unesc(v)); break;
                    case 'i': map.put(k, Integer.parseInt(v)); break;
                    case 'l': map.put(k, Long.parseLong(v)); break;
                    case 'f': map.put(k, Float.parseFloat(v)); break;
                    case 'b': map.put(k, Boolean.valueOf(v.equals("true"))); break;
                    case 'S': {
                        HashSet<String> set = new HashSet<String>();
                        if (v.length() > 0) for (String x : v.split("\u0002", -1)) set.add(unesc(x));
                        map.put(k, set);
                        break;
                    }
                }
            } catch (RuntimeException e) {}
        }
    }
    private void save() {
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, Object> e : map.entrySet()) {
            Object v = e.getValue();
            char t = v instanceof String ? 's' : v instanceof Integer ? 'i' : v instanceof Long ? 'l' : v instanceof Float ? 'f' : v instanceof Boolean ? 'b' : 'S';
            sb.append(t).append(esc(e.getKey())).append('\u0001');
            if (t == 's') sb.append(esc((String) v));
            else if (t == 'S') {
                boolean first = true;
                for (Object x : (Set<?>) v) { if (!first) sb.append('\u0002'); sb.append(esc(String.valueOf(x))); first = false; }
            } else sb.append(v);
            sb.append('\n');
        }
        VMStorage.save(file(), sb.toString());
    }
    private static String esc(String s) { return s.replace("\\", "\\\\").replace("\n", "\\n"); }
    private static String unesc(String s) {
        if (s.indexOf('\\') < 0) return s;
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '\\' && i + 1 < s.length()) { char n = s.charAt(++i); sb.append(n == 'n' ? '\n' : n); }
            else sb.append(c);
        }
        return sb.toString();
    }
    public Map<String, ?> getAll() { return new HashMap<String, Object>(map); }
    public String getString(String k, String d) { Object o = map.get(k); return o instanceof String ? (String) o : d; }
    @SuppressWarnings("unchecked")
    public Set<String> getStringSet(String k, Set<String> d) { Object o = map.get(k); return o instanceof Set ? (Set<String>) o : d; }
    public int getInt(String k, int d) { Object o = map.get(k); return o instanceof Integer ? (Integer) o : d; }
    public long getLong(String k, long d) { Object o = map.get(k); return o instanceof Long ? (Long) o : d; }
    public float getFloat(String k, float d) { Object o = map.get(k); return o instanceof Float ? (Float) o : d; }
    public boolean getBoolean(String k, boolean d) { Object o = map.get(k); return o instanceof Boolean ? (Boolean) o : d; }
    public boolean contains(String k) { return map.containsKey(k); }
    public void registerOnSharedPreferenceChangeListener(OnSharedPreferenceChangeListener l) { listeners.add(l); }
    public void unregisterOnSharedPreferenceChangeListener(OnSharedPreferenceChangeListener l) { listeners.remove(l); }
    public Editor edit() {
        return new Editor() {
            private final HashMap<String, Object> pending = new HashMap<String, Object>();
            private boolean clear;
            public Editor putString(String k, String v) { pending.put(k, v); return this; }
            public Editor putStringSet(String k, Set<String> v) { pending.put(k, v == null ? null : new HashSet<String>(v)); return this; }
            public Editor putInt(String k, int v) { pending.put(k, v); return this; }
            public Editor putLong(String k, long v) { pending.put(k, v); return this; }
            public Editor putFloat(String k, float v) { pending.put(k, v); return this; }
            public Editor putBoolean(String k, boolean v) { pending.put(k, v); return this; }
            public Editor remove(String k) { pending.put(k, this); return this; }
            public Editor clear() { clear = true; return this; }
            public boolean commit() {
                if (clear) map.clear();
                for (Map.Entry<String, Object> e : pending.entrySet()) {
                    if (e.getValue() == this || e.getValue() == null) map.remove(e.getKey());
                    else map.put(e.getKey(), e.getValue());
                    for (OnSharedPreferenceChangeListener l : listeners) l.onSharedPreferenceChanged(MemoryPreferences.this, e.getKey());
                }
                pending.clear();
                save();
                return true;
            }
            public void apply() { commit(); }
        };
    }
}
