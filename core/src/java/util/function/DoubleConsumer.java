package java.util.function;
public interface DoubleConsumer { void accept(double value); default DoubleConsumer andThen(final DoubleConsumer p){java.util.Objects.requireNonNull(p);return new DoubleConsumer(){public void accept(double v){DoubleConsumer.this.accept(v);p.accept(v);}};} }
