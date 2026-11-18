package java.util.concurrent;
public class CompletionException extends RuntimeException { protected CompletionException(){super();} protected CompletionException(String m){super(m);} public CompletionException(String m,Throwable t){super(m,t);} public CompletionException(Throwable t){super(t);} }
