package android.content;
public final class ComponentName {
    private final String pkg, cls;
    public ComponentName(String pkg, String cls) { this.pkg = pkg; this.cls = cls; }
    public ComponentName(Context ctx, String cls) { this(ctx.getPackageName(), cls); }
    public ComponentName(Context ctx, Class<?> cls) { this(ctx.getPackageName(), cls.getName()); }
    public String getPackageName() { return pkg; }
    public String getClassName() { return cls; }
    public String getShortClassName() { return cls.startsWith(pkg) ? cls.substring(pkg.length()) : cls; }
    public String flattenToString() { return pkg + "/" + cls; }
}
