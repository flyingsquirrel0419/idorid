package android.os;
public final class CancellationSignal {
    public interface OnCancelListener { void onCancel(); }
    private boolean canceled; private OnCancelListener listener;
    public boolean isCanceled() { return canceled; }
    public void throwIfCanceled() { if (canceled) throw new OperationCanceledException(); }
    public void cancel() { if (canceled) return; canceled=true; if (listener!=null) listener.onCancel(); }
    public void setOnCancelListener(OnCancelListener l) { listener=l; if (canceled && l!=null) l.onCancel(); }
}
