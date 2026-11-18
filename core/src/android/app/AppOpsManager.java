package android.app;
public class AppOpsManager { public static final int MODE_ALLOWED=0,MODE_IGNORED=1,MODE_ERRORED=2,MODE_DEFAULT=3; public int checkOpNoThrow(String op,int uid,String pkg) { return MODE_ALLOWED; } public int noteOpNoThrow(String op,int uid,String pkg) { return MODE_ALLOWED; } public static String permissionToOp(String permission) { return null; }  public int noteProxyOp(String op,String pkg) { return MODE_ALLOWED; } public int noteProxyOpNoThrow(String op,String pkg) { return MODE_ALLOWED; }
}
