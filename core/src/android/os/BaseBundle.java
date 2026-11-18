package android.os;
import java.util.Set;
public abstract class BaseBundle {
    protected BaseBundle() {}
    public abstract boolean containsKey(String key); public abstract boolean isEmpty(); public abstract int size(); public abstract Set<String> keySet();
    public abstract boolean getBoolean(String key); public abstract boolean getBoolean(String key,boolean fallback); public abstract int getInt(String key); public abstract int getInt(String key,int fallback); public abstract String getString(String key); public abstract String getString(String key,String fallback); public abstract String[] getStringArray(String key);
    public abstract void putBoolean(String key,boolean value); public abstract void putInt(String key,int value); public abstract void putString(String key,String value); public abstract void putStringArray(String key,String[] value);
}
