package android.app;

import android.content.Context;
import android.content.ContextWrapper;
import android.content.Intent;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.view.Window;
import android.widget.FrameLayout;

public class Activity extends ContextWrapper {
    public static final int RESULT_OK = -1, RESULT_CANCELED = 0, RESULT_FIRST_USER = 1;
    Intent mIntent;
    /** The window's decor view: the root laid out and serialized by ActivityThread. */
    FrameLayout mContent;
    CharSequence mTitle;
    boolean mFinished, mResumed;
    int mResultCode = RESULT_CANCELED;
    Intent mResultData;
    Activity mCaller;
    int mRequestCode = -1;
    private Window window;
    private Application app;

    public Activity() { super(null); }
    final void attach(Context base, Application app, Intent intent) {
        attachBaseContext(base);
        this.app = app;
        mIntent = intent;
        window = new Window(this);
        mContent = (FrameLayout) window.getDecorView();
    }
    protected void onCreate(Bundle savedInstanceState) {}
    protected void onPostCreate(Bundle savedInstanceState) {}
    protected void onStart() {}
    protected void onRestart() {}
    protected void onResume() {}
    protected void onPostResume() {}
    protected void onPause() {}
    protected void onStop() {}
    protected void onDestroy() {}
    protected void onSaveInstanceState(Bundle out) {}
    protected void onRestoreInstanceState(Bundle in) {}
    protected void onNewIntent(Intent i) {}
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {}
    public void onRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {}
    public void onBackPressed() { finish(); }
    public void onWindowFocusChanged(boolean hasFocus) {}
    public void onConfigurationChanged(android.content.res.Configuration c) {}
    public void onLowMemory() {}
    public void onUserInteraction() {}
    public boolean onCreateOptionsMenu(android.view.Menu menu) { return false; }
    public boolean onOptionsItemSelected(android.view.MenuItem item) { return false; }
    public boolean onKeyDown(int keyCode, android.view.KeyEvent e) { return false; }

    public void setContentView(int layoutResID) { getWindow().setContentView(layoutResID); onContentChanged(); }
    public void setContentView(View view) {
        ViewGroup.LayoutParams p = view.getLayoutParams();
        getWindow().setContentView(view, p != null ? p : new FrameLayout.LayoutParams(-1, -1));
        onContentChanged();
    }
    public void setContentView(View view, ViewGroup.LayoutParams p) { getWindow().setContentView(view, p); onContentChanged(); }
    public void addContentView(View view, ViewGroup.LayoutParams p) { getWindow().addContentView(view, p); onContentChanged(); }
    public void onContentChanged() {}
    @SuppressWarnings("unchecked")
    public <T extends View> T findViewById(int id) { return (T) getWindow().findViewById(id); }
    @SuppressWarnings("unchecked")
    public final <T extends View> T requireViewById(int id) {
        T v = findViewById(id);
        if (v == null) throw new IllegalArgumentException("ID does not reference a View inside this Activity");
        return v;
    }
    public Intent getIntent() { return mIntent; }
    public void setIntent(Intent i) { mIntent = i; }
    public void setTitle(CharSequence t) { mTitle = t; }
    public void setTitle(int resId) { mTitle = getText(resId); }
    public CharSequence getTitle() { return mTitle; }
    public void finish() { mFinished = true; }
    public void finishAffinity() { mFinished = true; }
    public boolean isFinishing() { return mFinished; }
    public boolean isDestroyed() { return false; }
    public void recreate() {}
    public final Application getApplication() { return app; }
    public Window getWindow() { return window; }
    public LayoutInflater getLayoutInflater() { return LayoutInflater.from(this); }
    public android.view.WindowManager getWindowManager() { return null; }
    public final void runOnUiThread(Runnable r) { r.run(); }
    public void startActivity(Intent intent) { ActivityThread.startActivity(this, intent, -1); }
    public void startActivity(Intent intent, Bundle options) { startActivity(intent); }
    public void startActivityForResult(Intent intent, int requestCode) { ActivityThread.startActivity(this, intent, requestCode); }
    public void startActivityForResult(Intent intent, int requestCode, Bundle options) { startActivityForResult(intent, requestCode); }
    public final void setResult(int code) { mResultCode = code; }
    public final void setResult(int code, Intent data) { mResultCode = code; mResultData = data; }
    public android.content.ComponentName getComponentName() { return new android.content.ComponentName(getPackageName(), getClass().getName()); }
    public android.view.MenuInflater getMenuInflater() { return new android.view.MenuInflater(this); }
    public void requestPermissions(String[] p, int code) {
        int[] r = new int[p.length];
        for (int i = 0; i < r.length; i++) r[i] = android.content.pm.PackageManager.PERMISSION_DENIED;
        onRequestPermissionsResult(code, p, r);
    }
    public boolean shouldShowRequestPermissionRationale(String p) { return false; }
    public final boolean requestWindowFeature(int f) { return true; }
    public void setRequestedOrientation(int o) {}
    public void overridePendingTransition(int a, int b) {}
    public void invalidateOptionsMenu() {}
    public ActionBar getActionBar() { return null; }
    public final void showDialog(int id) {}
    public void openOptionsMenu() {}
    private android.content.res.Resources.Theme theme;
    private int themeRes;
    private LayoutInflater inflater;
    public android.content.res.Resources.Theme getTheme() {
        if (theme == null) {
            theme = getResources().newTheme();
            int t = themeRes;
            if (t == 0) t = ActivityThread.nativeTheme(getClass().getName());
            if (t == 0) t = ActivityThread.nativeTheme("");
            theme.applyStyle(t != 0 ? t : IDORID_DEFAULT_THEME, true);
        }
        return theme;
    }
    public void setTheme(int resid) { themeRes = resid; if (theme != null) theme.applyStyle(resid, true); }
    public Object getSystemService(String name) {
        if (LAYOUT_INFLATER_SERVICE.equals(name)) { if (inflater == null) inflater = LayoutInflater.idoridCreate(this); return inflater; }
        return super.getSystemService(name);
    }
}
