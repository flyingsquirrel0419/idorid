package android.app;

import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.content.res.Resources;
import android.os.Looper;

final class ContextImpl extends Context {
    static final Resources RES = new Resources();
    Application app;
    public Resources getResources() { return RES; }
    public String getPackageName() { return ActivityThread.packageName; }
    public Context getApplicationContext() { return app != null ? app : this; }
    public SharedPreferences getSharedPreferences(String name, int mode) { return VMPrefs.get(name); }
    public Object getSystemService(String name) {
        if (LAYOUT_INFLATER_SERVICE.equals(name)) return android.view.LayoutInflater.from(this);
        if (INPUT_METHOD_SERVICE.equals(name)) return new android.view.inputmethod.InputMethodManager();
        return null;
    }
    public void startActivity(Intent intent) { ActivityThread.startActivity(null, intent, -1); }
    public Looper getMainLooper() { return Looper.getMainLooper(); }
    public java.io.File getFilesDir() { return new java.io.File("/data/files"); }
    public java.io.File getCacheDir() { return new java.io.File("/data/cache"); }
}
