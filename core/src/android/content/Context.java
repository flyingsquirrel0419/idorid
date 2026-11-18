package android.content;

import android.content.res.Resources;
import android.os.Looper;

public abstract class Context {
    public static final int MODE_PRIVATE = 0, MODE_APPEND = 0x8000;
    public static final String LAYOUT_INFLATER_SERVICE = "layout_inflater", INPUT_METHOD_SERVICE = "input_method", WINDOW_SERVICE = "window",
        CLIPBOARD_SERVICE = "clipboard", VIBRATOR_SERVICE = "vibrator", AUDIO_SERVICE = "audio", CONNECTIVITY_SERVICE = "connectivity",
        NOTIFICATION_SERVICE = "notification", ALARM_SERVICE = "alarm", SENSOR_SERVICE = "sensor", LOCATION_SERVICE = "location";
    public abstract Resources getResources();
    public abstract String getPackageName();
    public abstract Context getApplicationContext();
    public abstract SharedPreferences getSharedPreferences(String name, int mode);
    public abstract Object getSystemService(String name);
    public abstract void startActivity(Intent intent);
    public abstract Looper getMainLooper();
    public abstract java.io.File getFilesDir();
    public abstract java.io.File getCacheDir();
    public final String getString(int id) { return getResources().getString(id); }
    public final String getString(int id, Object... args) { return getResources().getString(id, args); }
    public final CharSequence getText(int id) { return getResources().getText(id); }
    public final int getColor(int id) { return getResources().getColor(id); }
    public android.content.res.Resources.Theme getTheme() { return getResources().newTheme(); }
    public void setTheme(int resid) {}
    public ClassLoader getClassLoader() { return ClassLoader.getSystemClassLoader(); }
    public android.content.pm.ApplicationInfo getApplicationInfo() { return new android.content.pm.ApplicationInfo(); }
    public android.content.pm.PackageManager getPackageManager() { return new android.content.pm.PackageManager(); }
    public ContentResolver getContentResolver() { return new ContentResolver(); }
    public void sendBroadcast(Intent i) {}
    public ComponentName startService(Intent i) { return null; }
    public boolean stopService(Intent i) { return false; }
    public int checkSelfPermission(String p) { return android.content.pm.PackageManager.PERMISSION_DENIED; }
}
