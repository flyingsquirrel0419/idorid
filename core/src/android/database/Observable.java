package android.database;
import java.util.ArrayList;
public abstract class Observable<T> {
    protected final ArrayList<T> mObservers=new ArrayList<T>();
    public void registerObserver(T observer) { if(observer==null)throw new IllegalArgumentException(); if(mObservers.contains(observer))throw new IllegalStateException(); mObservers.add(observer); }
    public void unregisterObserver(T observer) { if(observer==null)throw new IllegalArgumentException(); if(!mObservers.remove(observer))throw new IllegalStateException(); }
    public void unregisterAll() { mObservers.clear(); }
}
