package android.content;

import android.net.Uri;
import android.os.Bundle;

public class Intent implements Cloneable, android.os.Parcelable {
    public static final String ACTION_MAIN = "android.intent.action.MAIN", ACTION_VIEW = "android.intent.action.VIEW",
        ACTION_SEND = "android.intent.action.SEND", ACTION_DIAL = "android.intent.action.DIAL", CATEGORY_LAUNCHER = "android.intent.category.LAUNCHER",
        CATEGORY_DEFAULT = "android.intent.category.DEFAULT", EXTRA_TEXT = "android.intent.extra.TEXT", EXTRA_SUBJECT = "android.intent.extra.SUBJECT",
        EXTRA_EMAIL = "android.intent.extra.EMAIL";
    public static final int FLAG_ACTIVITY_NEW_TASK = 0x10000000, FLAG_ACTIVITY_CLEAR_TOP = 0x04000000, FLAG_ACTIVITY_SINGLE_TOP = 0x20000000,
        FLAG_ACTIVITY_CLEAR_TASK = 0x00008000, FLAG_ACTIVITY_NO_HISTORY = 0x40000000;
    private String action;
    private Uri data;
    private String type;
    private ComponentName component;
    private Bundle extras;
    private int flags;
    private java.util.HashSet<String> categories;
    public Intent() {}
    public Intent(String action) { this.action = action; }
    public Intent(String action, Uri uri) { this.action = action; this.data = uri; }
    public Intent(Context ctx, Class<?> cls) { component = new ComponentName(ctx, cls); }
    public Intent(Intent o) { action = o.action; data = o.data; type = o.type; component = o.component; flags = o.flags; if (o.extras != null) extras = new Bundle(o.extras); }
    public Intent setClass(Context ctx, Class<?> cls) { component = new ComponentName(ctx, cls); return this; }
    public Intent setClassName(String pkg, String cls) { component = new ComponentName(pkg, cls); return this; }
    public Intent setClassName(Context ctx, String cls) { component = new ComponentName(ctx, cls); return this; }
    public Intent setComponent(ComponentName c) { component = c; return this; }
    public ComponentName getComponent() { return component; }
    public String getAction() { return action; }
    public Intent setAction(String a) { action = a; return this; }
    public Uri getData() { return data; }
    public Intent setData(Uri u) { data = u; return this; }
    public String getDataString() { return data == null ? null : data.toString(); }
    public String getType() { return type; }
    public Intent setType(String t) { type = t; return this; }
    public Intent setDataAndType(Uri u, String t) { data = u; type = t; return this; }
    public int getFlags() { return flags; }
    public Intent setFlags(int f) { flags = f; return this; }
    public Intent addFlags(int f) { flags |= f; return this; }
    public Intent addCategory(String c) { if (categories == null) categories = new java.util.HashSet<String>(); categories.add(c); return this; }
    public boolean hasCategory(String c) { return categories != null && categories.contains(c); }
    public Intent setPackage(String p) { return this; }
    private Bundle ex() { if (extras == null) extras = new Bundle(); return extras; }
    public Bundle getExtras() { return extras == null ? null : new Bundle(extras); }
    public Intent putExtras(Bundle b) { ex().putAll(b); return this; }
    public Intent putExtras(Intent i) { if (i.extras != null) ex().putAll(i.extras); return this; }
    public boolean hasExtra(String k) { return extras != null && extras.containsKey(k); }
    public void removeExtra(String k) { if (extras != null) extras.remove(k); }
    public Intent putExtra(String k, String v) { ex().putString(k, v); return this; }
    public Intent putExtra(String k, CharSequence v) { ex().putCharSequence(k, v); return this; }
    public Intent putExtra(String k, int v) { ex().putInt(k, v); return this; }
    public Intent putExtra(String k, long v) { ex().putLong(k, v); return this; }
    public Intent putExtra(String k, boolean v) { ex().putBoolean(k, v); return this; }
    public Intent putExtra(String k, float v) { ex().putFloat(k, v); return this; }
    public Intent putExtra(String k, double v) { ex().putDouble(k, v); return this; }
    public Intent putExtra(String k, char v) { ex().putChar(k, v); return this; }
    public Intent putExtra(String k, String[] v) { ex().putStringArray(k, v); return this; }
    public Intent putExtra(String k, int[] v) { ex().putIntArray(k, v); return this; }
    public Intent putExtra(String k, Bundle v) { ex().putBundle(k, v); return this; }
    public Intent putExtra(String k, java.io.Serializable v) { ex().putSerializable(k, v); return this; }
    public Intent putExtra(String k, android.os.Parcelable v) { ex().putParcelable(k, v); return this; }
    public Intent putStringArrayListExtra(String k, java.util.ArrayList<String> v) { ex().putStringArrayList(k, v); return this; }
    public String getStringExtra(String k) { return extras == null ? null : extras.getString(k); }
    public CharSequence getCharSequenceExtra(String k) { return extras == null ? null : extras.getCharSequence(k); }
    public int getIntExtra(String k, int d) { return extras == null ? d : extras.getInt(k, d); }
    public long getLongExtra(String k, long d) { return extras == null ? d : extras.getLong(k, d); }
    public boolean getBooleanExtra(String k, boolean d) { return extras == null ? d : extras.getBoolean(k, d); }
    public float getFloatExtra(String k, float d) { return extras == null ? d : extras.getFloat(k, d); }
    public double getDoubleExtra(String k, double d) { return extras == null ? d : extras.getDouble(k, d); }
    public char getCharExtra(String k, char d) { return extras == null || !extras.containsKey(k) ? d : extras.getChar(k); }
    public String[] getStringArrayExtra(String k) { return extras == null ? null : extras.getStringArray(k); }
    public int[] getIntArrayExtra(String k) { return extras == null ? null : extras.getIntArray(k); }
    public Bundle getBundleExtra(String k) { return extras == null ? null : extras.getBundle(k); }
    public java.io.Serializable getSerializableExtra(String k) { return extras == null ? null : extras.getSerializable(k); }
    public <T extends android.os.Parcelable> T getParcelableExtra(String k) { return extras == null ? null : extras.<T>getParcelable(k); }
    public java.util.ArrayList<String> getStringArrayListExtra(String k) { return extras == null ? null : extras.getStringArrayList(k); }
    public static Intent createChooser(Intent target, CharSequence title) { return target; }
    public Object clone() { return new Intent(this); }
    public int describeContents() { return 0; }
    public void writeToParcel(android.os.Parcel p, int f) {}
    public String toString() { return "Intent { act=" + action + " cmp=" + (component == null ? null : component.flattenToString()) + " }"; }
}
