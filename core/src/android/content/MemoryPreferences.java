package android.content;

import java.util.*;

/** In-memory SharedPreferences (persistence is a TODO in the host bridge). */
final class MemoryPreferences implements SharedPreferences {
    private static final HashMap<String, MemoryPreferences> ALL = new HashMap<String, MemoryPreferences>();
    private final HashMap<String, Object> map = new HashMap<String, Object>();
    private final ArrayList<OnSharedPreferenceChangeListener> listeners = new ArrayList<OnSharedPreferenceChangeListener>();
    static MemoryPreferences get(String name) {
        MemoryPreferences p = ALL.get(name);
        if (p == null) { p = new MemoryPreferences(); ALL.put(name, p); }
        return p;
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
                return true;
            }
            public void apply() { commit(); }
        };
    }
}
