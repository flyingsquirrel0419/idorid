package java.util.function;
public interface Consumer<T> { void accept(T t); default Consumer<T> andThen(final Consumer<? super T> a){ java.util.Objects.requireNonNull(a); return new Consumer<T>(){public void accept(T t){Consumer.this.accept(t);a.accept(t);}}; } }
