package android.app;
final class VMPrefs {
    static android.content.SharedPreferences get(String name) { return android.content.VMPrefsAccess.get(name); }
}
