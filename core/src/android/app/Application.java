package android.app;
import android.content.ContextWrapper;
public class Application extends ContextWrapper {
    public interface ActivityLifecycleCallbacks {}
    public Application() { super(null); }
    public void onCreate() {}
    public void onTerminate() {}
    public void onLowMemory() {}
    public void onTrimMemory(int level) {}
    public void registerActivityLifecycleCallbacks(ActivityLifecycleCallbacks c) {}
    final void attach(android.content.Context base) { attachBaseContext(base); }
}
