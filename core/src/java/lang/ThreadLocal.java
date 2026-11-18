package java.lang;
public class ThreadLocal<T> {
    private boolean set;
    private T value;
    public ThreadLocal() {}
    protected T initialValue() { return null; }
    public T get() { if (!set) { value = initialValue(); set = true; } return value; }
    public void set(T v) { value = v; set = true; }
    public void remove() { set = false; value = null; }
    public static <S> ThreadLocal<S> withInitial(final java.util.function.Supplier<? extends S> s) {
        return new ThreadLocal<S>() { protected S initialValue() { return s.get(); } };
    }
}
