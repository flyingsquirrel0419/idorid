package java.util;
public interface Iterator<E> { boolean hasNext(); E next(); default void remove(){ throw new UnsupportedOperationException(); } default void forEachRemaining(java.util.function.Consumer<? super E> action){ while(hasNext()) action.accept(next()); } }
