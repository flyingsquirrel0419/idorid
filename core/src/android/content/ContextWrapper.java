package android.content;

import android.content.res.Resources;
import android.os.Looper;

public class ContextWrapper extends Context {
    Context mBase;
    public ContextWrapper(Context base) { mBase = base; }
    protected void attachBaseContext(Context base) { mBase = base; }
    public Context getBaseContext() { return mBase; }
    public Resources getResources() { return mBase.getResources(); }
    public String getPackageName() { return mBase.getPackageName(); }
    public Context getApplicationContext() { return mBase.getApplicationContext(); }
    public SharedPreferences getSharedPreferences(String n, int m) { return mBase.getSharedPreferences(n, m); }
    public Object getSystemService(String n) { return mBase.getSystemService(n); }
    public void startActivity(Intent i) { mBase.startActivity(i); }
    public Looper getMainLooper() { return Looper.getMainLooper(); }
    public java.io.File getFilesDir() { return mBase.getFilesDir(); }
    public java.io.File getCacheDir() { return mBase.getCacheDir(); }
}
