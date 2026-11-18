package android.app;

import android.content.Intent;
import android.os.Looper;
import android.view.Json;
import android.view.View;
import android.view.VMViews;
import java.util.ArrayList;

/**
 * idorid host entry points. The Rust side calls these static methods; each
 * returns the UI JSON described in docs/PLAN.md.
 */
public final class ActivityThread {
    static String packageName = "";
    static String appLabel = "";
    private static Application app;
    private static ContextImpl base;
    private static final ArrayList<Activity> stack = new ArrayList<Activity>();
    private static final ArrayList<String> toasts = new ArrayList<String>();
    private static final ArrayList<Dialog> dialogs = new ArrayList<Dialog>();
    private static String error;
    private static boolean allFinished;

    private ActivityThread() {}

    public static String launch(String pkg, String label, String activityClass, String appClass) {
        packageName = pkg;
        appLabel = label;
        base = new ContextImpl();
        try {
            app = appClass != null && appClass.length() > 0 ? (Application) Class.forName(appClass).newInstance() : new Application();
            app.attach(base);
            base.app = app;
            app.onCreate();
            Intent i = new Intent(Intent.ACTION_MAIN);
            i.setClassName(pkg, activityClass);
            i.addCategory(Intent.CATEGORY_LAUNCHER);
            startActivity(null, i, -1);
        } catch (Throwable t) { fail(t); }
        return settle();
    }

    static void startActivity(Activity caller, Intent intent, int requestCode) {
        if (intent.getComponent() == null) {
            toast("No app can handle " + (intent.getAction() == null ? "this action" : intent.getAction()));
            return;
        }
        String cls = intent.getComponent().getClassName();
        Activity a;
        try { a = (Activity) Class.forName(cls).newInstance(); }
        catch (Throwable t) { throw new RuntimeException("Unable to instantiate activity " + cls, t); }
        a.attach(base, app, intent);
        a.mCaller = caller;
        a.mRequestCode = requestCode;
        a.mTitle = appLabel;
        Activity top = top();
        if (top != null && top.mResumed) { top.mResumed = false; top.onPause(); }
        stack.add(a);
        a.onCreate(null);
        a.onPostCreate(null);
        if (!a.mFinished) {
            a.onStart();
            a.onResume();
            a.onPostResume();
            a.mResumed = true;
            if (top != null) top.onStop();
        }
    }

    private static Activity top() { return stack.isEmpty() ? null : stack.get(stack.size() - 1); }

    /** Pops finished activities, resumes the new top, runs queued work. */
    private static String settle() {
        try {
            for (int guard = 0; guard < 100; guard++) {
                Looper.drain();
                boolean changed = false;
                for (int i = stack.size() - 1; i >= 0; i--) {
                    Activity a = stack.get(i);
                    if (!a.mFinished) continue;
                    changed = true;
                    stack.remove(i);
                    if (a.mResumed) { a.mResumed = false; a.onPause(); }
                    a.onStop();
                    a.onDestroy();
                    if (a.mCaller != null && a.mRequestCode >= 0 && stack.contains(a.mCaller)) a.mCaller.onActivityResult(a.mRequestCode, a.mResultCode, a.mResultData);
                }
                Activity t = top();
                if (t != null && !t.mResumed && !t.mFinished) {
                    changed = true;
                    t.onRestart();
                    t.onStart();
                    t.onResume();
                    t.mResumed = true;
                }
                if (!changed) break;
            }
            if (stack.isEmpty()) allFinished = true;
        } catch (Throwable t) { fail(t); }
        return dump(Looper.drain());
    }

    private static void fail(Throwable t) {
        error = t.toString();
        android.util.Log.e("AndroidRuntime", "FATAL EXCEPTION: main", t);
    }

    public static String click(int uid) {
        try {
            if (!dialogs.isEmpty() && uid < 0 && uid >= -1000) {
                dialogs.get(dialogs.size() - 1).idoridButton(uid <= -100 ? -uid - 100 : uid);
            } else {
                View v = find(uid);
                if (v != null && v.isEnabled()) v.performClick();
            }
        } catch (Throwable t) { fail(t); }
        return settle();
    }

    public static String longClick(int uid) {
        try { View v = find(uid); if (v != null) v.performLongClick(); } catch (Throwable t) { fail(t); }
        return settle();
    }

    public static String setText(int uid, String text) {
        try {
            View v = find(uid);
            if (v instanceof android.widget.TextView) ((android.widget.TextView) v).idoridUserText(text);
        } catch (Throwable t) { fail(t); }
        return settle();
    }

    public static String editorAction(int uid) {
        try { View v = find(uid); if (v instanceof android.widget.TextView) ((android.widget.TextView) v).idoridEditorAction(); } catch (Throwable t) { fail(t); }
        return settle();
    }

    public static String setProgress(int uid, int p) {
        try { View v = find(uid); if (v instanceof android.widget.SeekBar) ((android.widget.SeekBar) v).idoridUserProgress(p); } catch (Throwable t) { fail(t); }
        return settle();
    }

    public static String back() {
        try {
            if (!dialogs.isEmpty()) {
                Dialog d = dialogs.get(dialogs.size() - 1);
                if (d.cancelable) d.cancel();
            } else {
                Activity t = top();
                if (t != null) t.onBackPressed();
            }
        } catch (Throwable t) { fail(t); }
        return settle();
    }

    /** Timer callback from the host for delayed Handler messages. */
    public static String tick() { return settle(); }

    public static String ui() { return dump(Looper.drain()); }

    private static View find(int uid) {
        for (int i = dialogs.size() - 1; i >= 0; i--) {
            View c = dialogs.get(i).content;
            if (c != null) { View v = VMViews.byUid(c, uid); if (v != null) return v; }
        }
        Activity t = top();
        return t == null ? null : VMViews.byUid(t.mContent, uid);
    }

    public static void toast(String s) { toasts.add(s); }
    static void showDialog(Dialog d) { if (!dialogs.contains(d)) dialogs.add(d); }
    static void dismissDialog(Dialog d) { dialogs.remove(d); }

    private static String dump(long nextMs) {
        StringBuilder sb = new StringBuilder(1024);
        Activity t = top();
        sb.append("{\"title\":");
        Json.str(sb, t == null ? appLabel : (t.mTitle == null ? appLabel : t.mTitle));
        sb.append(",\"finished\":").append(allFinished || t == null);
        sb.append(",\"error\":");
        Json.str(sb, error);
        error = null;
        sb.append(",\"next\":").append(nextMs);
        sb.append(",\"depth\":").append(stack.size());
        if (t != null) { sb.append(",\"root\":"); t.mContent.idoridJson(sb); }
        if (!toasts.isEmpty()) {
            sb.append(",\"toasts\":[");
            for (int i = 0; i < toasts.size(); i++) { if (i > 0) sb.append(','); Json.str(sb, toasts.get(i)); }
            sb.append(']');
            toasts.clear();
        }
        if (!dialogs.isEmpty()) { sb.append(",\"dialog\":"); dialogs.get(dialogs.size() - 1).idoridJson(sb); }
        sb.append('}');
        return sb.toString();
    }
}
