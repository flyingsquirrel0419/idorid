package android.app;

import android.content.Context;
import android.content.DialogInterface;
import android.view.View;

public class Dialog implements DialogInterface {
    final Context ctx;
    CharSequence title;
    View content;
    boolean cancelable = true, showing;
    OnCancelListener onCancel;
    OnDismissListener onDismiss;
    public Dialog(Context c) { ctx = c; }
    public Dialog(Context c, int theme) { ctx = c; }
    public final Context getContext() { return ctx; }
    public void setTitle(CharSequence t) { title = t; }
    public void setTitle(int r) { title = ctx.getText(r); }
    public void setContentView(View v) { content = v; }
    public void setContentView(int layout) { content = android.view.LayoutInflater.from(ctx).inflate(layout, null); }
    @SuppressWarnings("unchecked")
    public <T extends View> T findViewById(int id) { return content == null ? null : (T) content.findViewById(id); }
    public void setCancelable(boolean c) { cancelable = c; }
    public void setCanceledOnTouchOutside(boolean c) {}
    public void setOnCancelListener(OnCancelListener l) { onCancel = l; }
    public void setOnDismissListener(OnDismissListener l) { onDismiss = l; }
    public void show() { showing = true; ActivityThread.showDialog(this); }
    public boolean isShowing() { return showing; }
    public void dismiss() {
        if (!showing) return;
        showing = false;
        ActivityThread.dismissDialog(this);
        if (onDismiss != null) onDismiss.onDismiss(this);
    }
    public void cancel() { if (onCancel != null) onCancel.onCancel(this); dismiss(); }
    public void hide() { dismiss(); }
    public android.view.Window getWindow() { return new android.view.Window(); }
    /** idorid: host button press (-1 positive, -2 negative, -3 neutral, >= 0 list item). */
    void idoridButton(int which) { dismiss(); }
    void idoridJson(StringBuilder sb) {
        sb.append("{\"title\":"); android.view.Json.str(sb, title);
        if (content != null) { sb.append(",\"content\":"); content.idoridJson(sb); }
        sb.append(",\"cancelable\":").append(cancelable);
        sb.append('}');
    }
}
