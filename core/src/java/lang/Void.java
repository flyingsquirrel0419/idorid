package java.lang;
public final class Void {
    @SuppressWarnings("unchecked")
    public static final Class<Void> TYPE = (Class<Void>) VMPrimitive.get('V');
    private Void() {}
}
