package android.view;
final class VMInflate {
    private VMInflate() {}
    /** new c(Context, AttributeSet) */
    static native Object construct(Class<?> c, Object ctx, Object attrs);
}
