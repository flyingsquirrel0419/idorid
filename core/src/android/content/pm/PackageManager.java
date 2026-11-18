package android.content.pm;
public class PackageManager {
    public static final int PERMISSION_GRANTED = 0, PERMISSION_DENIED = -1, GET_META_DATA = 0x80;
    public static class NameNotFoundException extends Exception { public NameNotFoundException() {} public NameNotFoundException(String m) { super(m); } }
    public PackageInfo getPackageInfo(String pkg, int flags) throws NameNotFoundException { return new PackageInfo(); }
    public ApplicationInfo getApplicationInfo(String pkg, int flags) throws NameNotFoundException { return new ApplicationInfo(); }
    public boolean hasSystemFeature(String f) { return false; }
}
