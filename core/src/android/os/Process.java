package android.os;
public final class Process { private Process() {} public static int myPid() { return 1; } public static int myUid() { return 10000; } public static int myTid() { return 1; } public static void sendSignal(int pid,int signal) {} public static void setThreadPriority(int priority) {} }
