package android.view;
public class KeyCharacterMap {
    public static class KeyData { public char displayLabel,number; public char[] meta=new char[4]; }
    public static KeyCharacterMap load(int deviceId) { return new KeyCharacterMap(); }
    public boolean getKeyData(int keyCode,KeyData data) { return false; }
    public int getKeyboardType() { return 2; }
    public int get(int keyCode,int metaState) { return 0; }
    public char getDisplayLabel(int keyCode) { return 0; }
    public char getNumber(int keyCode) { return 0; }
}
