package android.content.pm;
public class ResolveInfo { public ActivityInfo activityInfo; public ServiceInfo serviceInfo; public ProviderInfo providerInfo; public int priority, match;  public int specificIndex; public android.graphics.drawable.Drawable loadIcon(PackageManager pm) { return null; } public CharSequence loadLabel(PackageManager pm) { return activityInfo==null?null:activityInfo.name; }
}
