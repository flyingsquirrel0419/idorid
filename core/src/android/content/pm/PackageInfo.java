package android.content.pm;
public class PackageInfo { public String packageName="", versionName="1.0"; public int versionCode=1; public long firstInstallTime,lastUpdateTime; public Signature[] signatures; public ApplicationInfo applicationInfo; public long getLongVersionCode() { return versionCode; } }
