package android.system;
public final class OsConstants { public static final int S_IFMT=0170000,S_IFREG=0100000,S_IFDIR=0040000,ENOENT=2; public static boolean S_ISREG(int mode){return (mode&S_IFMT)==S_IFREG;} public static boolean S_ISDIR(int mode){return (mode&S_IFMT)==S_IFDIR;} }
