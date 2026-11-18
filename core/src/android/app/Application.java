package android.app;

import android.content.ContextWrapper;
import android.os.Bundle;
import java.util.ArrayList;

public class Application extends ContextWrapper implements android.content.ComponentCallbacks2 {
    public interface ActivityLifecycleCallbacks {
        default void onActivityPreCreated(Activity a, Bundle b) {}
        void onActivityCreated(Activity a, Bundle b);
        default void onActivityPostCreated(Activity a, Bundle b) {}
        default void onActivityPreStarted(Activity a) {}
        void onActivityStarted(Activity a);
        default void onActivityPostStarted(Activity a) {}
        default void onActivityPreResumed(Activity a) {}
        void onActivityResumed(Activity a);
        default void onActivityPostResumed(Activity a) {}
        default void onActivityPrePaused(Activity a) {}
        void onActivityPaused(Activity a);
        default void onActivityPostPaused(Activity a) {}
        default void onActivityPreStopped(Activity a) {}
        void onActivityStopped(Activity a);
        default void onActivityPostStopped(Activity a) {}
        default void onActivityPreSaveInstanceState(Activity a, Bundle b) {}
        void onActivitySaveInstanceState(Activity a, Bundle b);
        default void onActivityPostSaveInstanceState(Activity a, Bundle b) {}
        default void onActivityPreDestroyed(Activity a) {}
        void onActivityDestroyed(Activity a);
        default void onActivityPostDestroyed(Activity a) {}
    }
    public interface OnProvideAssistDataListener { void onProvideAssistData(Activity a, Bundle data); }
    final ArrayList<ActivityLifecycleCallbacks> callbacks = new ArrayList<ActivityLifecycleCallbacks>();
    public Application() { super(null); }
    public void onCreate() {}
    public void onTerminate() {}
    public void onLowMemory() {}
    public void onTrimMemory(int level) {}
    public void onConfigurationChanged(android.content.res.Configuration c) {}
    public void registerActivityLifecycleCallbacks(ActivityLifecycleCallbacks c) { synchronized (callbacks) { callbacks.add(c); } }
    public void unregisterActivityLifecycleCallbacks(ActivityLifecycleCallbacks c) { synchronized (callbacks) { callbacks.remove(c); } }
    public void registerComponentCallbacks(android.content.ComponentCallbacks c) {}
    public void unregisterComponentCallbacks(android.content.ComponentCallbacks c) {}
    public void registerOnProvideAssistDataListener(OnProvideAssistDataListener l) {}
    public void unregisterOnProvideAssistDataListener(OnProvideAssistDataListener l) {}
    public static String getProcessName() { return ActivityThread.packageName; }
    final void attach(android.content.Context base) { attachBaseContext(base); }
}
