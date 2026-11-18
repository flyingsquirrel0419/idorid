package java.lang.reflect;
public class AccessibleObject { private boolean accessible; public void setAccessible(boolean flag){accessible=flag;} public boolean isAccessible(){return accessible;} public static void setAccessible(AccessibleObject[] objects,boolean flag){for(AccessibleObject o:objects)o.setAccessible(flag);} }
