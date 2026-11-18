package android.content;
public final class VMPrefsAccess {
    private VMPrefsAccess() {}
    public static SharedPreferences get(String name) { return MemoryPreferences.get(name); }
}
