package android.os;
public class Binder implements IBinder { public static int getCallingPid(){return Process.myPid();} public static int getCallingUid(){return Process.myUid();} public static long clearCallingIdentity(){return 0;} public static void restoreCallingIdentity(long token){} public IBinder asBinder(){return this;} }
