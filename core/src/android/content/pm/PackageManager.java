package android.content.pm;
public class PackageManager {
    public static final int PERMISSION_GRANTED = 0, PERMISSION_DENIED = -1, GET_META_DATA = 0x80;
    public static class NameNotFoundException extends Exception { public NameNotFoundException() {} public NameNotFoundException(String m) { super(m); } }
    public PackageInfo getPackageInfo(String pkg, int flags) throws NameNotFoundException { PackageInfo p=new PackageInfo(); p.packageName=pkg; p.applicationInfo=getApplicationInfo(pkg,flags); return p; }
    public ApplicationInfo getApplicationInfo(String pkg, int flags) throws NameNotFoundException { ApplicationInfo a=new ApplicationInfo(); a.packageName=pkg; return a; }
    public boolean hasSystemFeature(String f) { return false; }
    public static class PackageInfoFlags { private final long value; private PackageInfoFlags(long v) { value=v; } public static PackageInfoFlags of(long v) { return new PackageInfoFlags(v); } public long getValue() { return value; } }
    public PackageInfo getPackageInfo(String pkg,PackageInfoFlags flags) throws NameNotFoundException { return getPackageInfo(pkg,(int)flags.getValue()); }
    public ActivityInfo getActivityInfo(android.content.ComponentName c,int flags) throws NameNotFoundException { ActivityInfo i=new ActivityInfo(); i.name=c.getClassName(); i.packageName=c.getPackageName(); return i; }
    public ProviderInfo getProviderInfo(android.content.ComponentName c,int flags) throws NameNotFoundException { ProviderInfo i=new ProviderInfo(); i.name=c.getClassName(); i.packageName=c.getPackageName(); return i; }
    public ServiceInfo getServiceInfo(android.content.ComponentName c,int flags) throws NameNotFoundException { ServiceInfo i=new ServiceInfo(); i.name=c.getClassName(); i.packageName=c.getPackageName(); return i; }
    public int getComponentEnabledSetting(android.content.ComponentName c) { return 0; } public void setComponentEnabledSetting(android.content.ComponentName c,int state,int flags) {}
    public String[] getPackagesForUid(int uid) { return new String[0]; }
    public java.util.List<ResolveInfo> queryIntentActivities(android.content.Intent i,int flags) { return new java.util.ArrayList<ResolveInfo>(); }
    public java.util.List<ResolveInfo> queryIntentActivityOptions(android.content.ComponentName c,android.content.Intent[] specifics,android.content.Intent i,int flags) { return new java.util.ArrayList<ResolveInfo>(); }
    public java.util.List<ResolveInfo> queryIntentContentProviders(android.content.Intent i,int flags) { return new java.util.ArrayList<ResolveInfo>(); }
    public ProviderInfo resolveContentProvider(String authority,int flags) { return null; }
    public ResolveInfo resolveActivity(android.content.Intent i,int flags) { return null; }
}
